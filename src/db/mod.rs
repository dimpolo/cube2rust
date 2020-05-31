use crate::*;
use regex::Regex;

pub fn get_alternate_function(
    mcu_family: MCUFamily,
    gpio: &GpioPin,
    peripheral_function: &str,
) -> u8 {
    let map = match mcu_family {
        MCUFamily::STM32F0 => &af_f0::AF_MAP,
        _ => todo!("other AF_MAPs"),
    };

    *map.get(peripheral_function)
        .unwrap_or_else(|| todo!("Alternate functions for {}", peripheral_function))
        .get(gpio.register.as_str())
        .unwrap_or_else(|| {
            todo!(
                "Alternate function for {}:{}",
                gpio.register,
                peripheral_function
            )
        })
}

pub struct MemSize {
    pub flash: usize,
    pub ram: usize,
}

pub fn get_mem_size(config: &Config) -> &MemSize {
    let map = match config.mcu_family {
        MCUFamily::STM32F0 => &mem_f0::MEMORY_SIZES,
        _ => todo!("other MEMORY_SIZES"),
    };

    map.get(config.mcu_name.as_str())
        .unwrap_or_else(|| todo!("Unknown MCU {}", config.mcu_name))
}

pub fn get_feature(config: &Config) -> anyhow::Result<&'static str> {
    let mcu_name = config.mcu_name.to_ascii_lowercase();

    let features = match config.mcu_family {
        MCUFamily::STM32F0 => features::F0_FEATURES,
        _ => todo!("More features"),
    };

    for feature in features {
        // x can be any word character
        // "stm32f030x4" -> Regex::new(r"^stm32f030\w4")
        let regex = Regex::new(&("^".to_string() + &feature.replace("x", r"\w"))).unwrap();

        if regex.is_match(&mcu_name) {
            return Ok(feature);
        }
    }

    bail!("no feature for {}", mcu_name)
}

mod af_f0;
mod features;
mod mem_f0;
