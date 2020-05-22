use crate::*;

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

mod af_f0;
mod mem_f0;
