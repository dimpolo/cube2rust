use crate::*;

pub fn get_rcc(config: &ConfigParams<'_>) -> anyhow::Result<RCC> {
    let rcc_params = config
        .get("RCC")
        .ok_or_else(|| anyhow!("No RCC configuration found"))?;

    // SYSCLK Config
    let sys_clock_source = parse_optional_param(rcc_params, "SYSCLKSource")?;
    let pll_clock_source = parse_optional_param(rcc_params, "PLLSourceVirtual")?;
    // ioc file does not set SYSCLKFreq_VALUE if it's HSI 8Mhz
    let sysclk_freq = parse_optional_u32(rcc_params, "SYSCLKFreq_VALUE")?;
    let hclk_freq = parse_optional_u32(rcc_params, "HCLKFreq_Value")?;
    let apb1_freq = parse_optional_u32(rcc_params, "APB1Freq_Value")?;

    let clock_source = get_clock_source(&sys_clock_source, &pll_clock_source, config)?;

    Ok(RCC {
        clock_source,
        sysclk_freq,
        hclk_freq,
        apb1_freq,
    })
}

fn get_clock_source(
    sys_clock_source: &Option<SYSCLKSourceType>,
    pll_clock_source: &Option<PLLSourceType>,
    config: &ConfigParams<'_>,
) -> anyhow::Result<ClockSource> {
    match sys_clock_source {
        None | Some(SYSCLKSourceType::RCC_SYSCLKSOURCE_HSI) => Ok(ClockSource::HSI),
        Some(SYSCLKSourceType::RCC_SYSCLKSOURCE_HSI48) => Ok(ClockSource::HSI48),
        Some(SYSCLKSourceType::RCC_SYSCLKSOURCE_HSE) => Ok(ClockSource::HSE(get_hse_mode(config)?)),
        Some(SYSCLKSourceType::RCC_SYSCLKSOURCE_PLLCLK) => match pll_clock_source {
            None | Some(PLLSourceType::RCC_PLLSOURCE_HSI) => Ok(ClockSource::HSI),
            Some(PLLSourceType::RCC_PLLSOURCE_HSI48) => Ok(ClockSource::HSI48),
            Some(PLLSourceType::RCC_PLLSOURCE_HSE) => Ok(ClockSource::HSE(get_hse_mode(config)?)),
        },
    }
}

fn get_hse_mode(config: &ConfigParams<'_>) -> anyhow::Result<HSEMode> {
    // RCC existance was checked already
    let rcc_params = config.get("RCC").unwrap();

    // get mode from pin configuration
    let &mode = config
        .get("PF0-OSC_IN")
        .map(|pfo| pfo.get("Mode"))
        .ok_or_else(|| anyhow!("PF0-OSC_IN required"))?
        .ok_or_else(|| anyhow!("PF0-OSC_IN.Mode required"))?;
    // get freq
    let freq = parse_mandatory_u32(rcc_params, "VCOOutput2Freq_Value")?;

    let mode: anyhow::Result<HSEMode> = match mode {
        "HSE-External-Oscillator" => Ok(HSEMode::NotBypassed(freq)),
        "HSE-External-Clock-Source" => Ok(HSEMode::Bypassed(freq)),
        _ => bail!("Unknown clock source"),
    };
    mode.context("Parsing of external clock source")
}

#[derive(Debug)]
pub struct RCC {
    // TODO USBClockSource
    // TODO CRS
    pub clock_source: ClockSource,
    pub sysclk_freq: Option<u32>,
    pub hclk_freq: Option<u32>,
    pub apb1_freq: Option<u32>,
}

#[derive(Debug, PartialEq)]
pub enum ClockSource {
    HSI,
    HSI48,
    HSE(HSEMode),
}

#[derive(Debug, PartialEq)]
pub enum HSEMode {
    NotBypassed(u32),
    Bypassed(u32),
}

parameter!(
    SYSCLKSourceType,
    [
        RCC_SYSCLKSOURCE_HSI,
        RCC_SYSCLKSOURCE_HSI48,
        RCC_SYSCLKSOURCE_PLLCLK,
        RCC_SYSCLKSOURCE_HSE
    ],
    default = RCC_SYSCLKSOURCE_HSI
);

parameter!(
    PLLSourceType,
    [RCC_PLLSOURCE_HSI, RCC_PLLSOURCE_HSI48, RCC_PLLSOURCE_HSE]
);
