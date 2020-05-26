use crate::*;
use std::convert::TryFrom;

const MAX_SPIS: u8 = 6;

#[derive(Debug)]
pub struct SPI {
    pub name_lower: String,
    pub name_upper: String,
    pub phase: Option<CLKPhase>,
    pub polarity: Option<CLKPolarity>,
    pub prescaler: BaudRatePrescaler,
    pub baudrate: BaudRate,
}

pub fn get_spis(config: &ConfigParams<'_>) -> anyhow::Result<Vec<SPI>> {
    let mut spis = Vec::new();

    for i in 1..=MAX_SPIS {
        let name_lower = f!("spi{i}");
        let name_upper = name_lower.to_ascii_uppercase();

        // search config for SPI1, SPI2, etc..
        if let Some(spi_params) = config.get::<str>(&name_upper) {
            let prescaler = parse_mandatory_param(spi_params, "BaudRatePrescaler")?;
            let baudrate = parse_mandatory_param(spi_params, "CalculateBaudRate")?;
            let phase = parse_optional_param(spi_params, "CLKPhase")?;
            let polarity = parse_optional_param(spi_params, "CLKPolarity")?;

            spis.push(SPI {
                name_lower,
                name_upper,
                phase,
                polarity,
                prescaler,
                baudrate,
            });
        }
    }
    Ok(spis)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BaudRate(pub u32);

impl TryFrom<&str> for BaudRate {
    type Error = anyhow::Error;

    fn try_from(string: &str) -> Result<Self, Self::Error> {
        // string looks like this 3.0 MBits/s
        let parts: Vec<&str> = string.split(' ').collect();

        let (&freq, &unit) = match &parts[..] {
            [freq, unit] => (freq, unit),
            _ => bail!("{}", string),
        };

        let freq: f32 = freq
            .parse()
            .map_err(|_| anyhow!("{} invalid Baudrate", string))?;

        let freq = match unit {
            "MBits/s" => freq * 1_000_000f32,
            "kBits/s" => freq * 1_000f32,
            "Bits/s" => freq,
            _ => bail!("Unknown unit {}", string),
        };

        Ok(BaudRate(freq as u32))
    }
}

parameter!(
    CLKPhase,
    [SPI_PHASE_1EDGE, SPI_PHASE_2EDGE],
    default = SPI_PHASE_1EDGE
);

parameter!(
    CLKPolarity,
    [SPI_POLARITY_LOW, SPI_POLARITY_HIGH],
    default = SPI_POLARITY_LOW
);

parameter!(
    BaudRatePrescaler,
    [
        SPI_BAUDRATEPRESCALER_2,
        SPI_BAUDRATEPRESCALER_4,
        SPI_BAUDRATEPRESCALER_8,
        SPI_BAUDRATEPRESCALER_16,
        SPI_BAUDRATEPRESCALER_32,
        SPI_BAUDRATEPRESCALER_64,
        SPI_BAUDRATEPRESCALER_128,
        SPI_BAUDRATEPRESCALER_256
    ]
);
