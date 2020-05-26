use std::convert::TryFrom;

use regex::Regex;

use crate::*;

#[derive(Debug)]
pub struct GpioPin {
    pub port: String,
    pub register: String,
    pub signal: SignalType,
    pub label: Option<String>,
    pub pin_state: Option<PinStateType>,
    pub speed: Option<SpeedType>,
    pub pu_pd: Option<PullType>,
    pub mode_default_output_pp: Option<ModeOutputType>,
}

pub fn get_gpios(config: &ConfigParams<'_>) -> anyhow::Result<(Vec<char>, Vec<GpioPin>)> {
    // regex matches PA11, PB4, etc
    let re = Regex::new(r"^P[A-K]\d{1,2}").unwrap();

    // collect regex matches into a params Vec<(match, params)>
    let mut gpio_params = Vec::new();
    for (name, params) in config {
        if let Some(name_match) = re.find(name) {
            gpio_params.push((name_match.as_str(), params));
        }
    }

    // sort vec alphanumerically, e.g. PA1, PA2, PA11, PB1
    gpio_params.sort_by(|(a, _), (b, _)| human_sort::compare(a, b));

    // map params to GpioPins
    let mut gpios = Vec::new();
    let mut ports = Vec::new();
    for (name, parameters) in gpio_params {
        let gpio: GpioPin = GpioPin::new(name, parameters).context(format!("Pin: {}", name))?;

        // Don't count external clock sources as GPIOs
        if let SignalType::Peripheral(ref signal) = gpio.signal {
            if signal == "RCC_OSC_OUT" || signal == "RCC_OSC_IN" {
                continue;
            }
        };

        ports.push(gpio.port.chars().last().unwrap());
        gpios.push(gpio);
    }

    ports.dedup();

    Ok((ports, gpios))
}

impl GpioPin {
    pub fn new(name: &str, parameters: &HashMap<&str, &str>) -> anyhow::Result<Self> {
        let (port, register) = parse_name(name)?;

        let signal = parse_mandatory_param(parameters, "Signal")?;
        let label = parameters.get("GPIO_Label").map(|&s| String::from(s));

        let pin_state = parse_optional_param(parameters, "PinState")?;
        let pu_pd = parse_optional_param(parameters, "GPIO_PuPd")?;

        let speed = parse_optional_param(parameters, "GPIO_Speed")?;
        let mode_default_output_pp = parse_optional_param(parameters, "GPIO_ModeDefaultOutputPP")?;

        Ok(GpioPin {
            port,
            register,
            signal,
            label,
            pin_state,
            speed,
            pu_pd,
            mode_default_output_pp,
        })
    }

    pub fn get_name(&self) -> String {
        let register = &self.register;

        match &self.label {
            Some(label) => label.clone(),
            None => match self.signal {
                SignalType::Peripheral(ref name) => name.to_lowercase(),
                SignalType::AdcInput => f!("adc_{register}"),
                _ => f!("gpio_{register}"),
            },
        }
    }
}

fn parse_name(name: &str) -> anyhow::Result<(String, String)> {
    let port_char_upper = name
        .chars()
        .nth(1)
        .ok_or_else(|| anyhow!("could not parse port"))?;
    let pin: u8 = name[2..]
        .parse()
        .map_err(|_| anyhow!("could not parse pin number"))?;

    let port_char_lower = port_char_upper.to_ascii_lowercase();
    let port = f!("GPIO{port_char_upper}");
    let register = f!("p{port_char_lower}{pin}");
    Ok((port, register))
}

#[derive(Debug, PartialEq)]
pub enum SignalType {
    GpioInput,
    GpioOutput,
    AdcInput,
    Peripheral(String),
}

impl TryFrom<&str> for SignalType {
    type Error = anyhow::Error;

    fn try_from(text: &str) -> anyhow::Result<Self, Self::Error> {
        match text {
            "GPIO_Input" => Ok(SignalType::GpioInput),
            "GPIO_Output" => Ok(SignalType::GpioOutput),
            "GPIO_Analog" => Ok(SignalType::AdcInput),
            _ => Ok(SignalType::Peripheral(String::from(text))),
        }
    }
}

parameter!(PinStateType, [GPIO_PIN_SET, GPIO_PIN_RESET]);
parameter!(
    PullType,
    [GPIO_PULLUP, GPIO_PULLDOWN, GPIO_NOPULL],
    default = GPIO_NOPULL
);
parameter!(
    ModeOutputType,
    [GPIO_MODE_OUTPUT_OD, GPIO_MODE_OUTPUT_PP],
    default = GPIO_MODE_OUTPUT_PP
);
parameter!(
    SpeedType,
    [
        GPIO_SPEED_FREQ_LOW,
        GPIO_SPEED_FREQ_MEDIUM,
        GPIO_SPEED_FREQ_HIGH,
        GPIO_SPEED_FREQ_VERY_HIGH
    ],
    default = GPIO_SPEED_FREQ_LOW
);
