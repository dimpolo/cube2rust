use crate::*;
use regex::Regex;

#[derive(Debug)]
pub struct I2C {
    pub name_lower: String,
    pub name_upper: String,
    pub mode: Option<Mode>,
}

pub fn get_i2cs(config: &ConfigParams<'_>) -> anyhow::Result<Vec<I2C>> {
    let mut i2cs = Vec::new();

    // regex matches I2C1_SDA, I2C2_SDA, etc
    let re = Regex::new(r"^(I2C\d)_SDA").unwrap();

    // if the I2C is left at default values CubeMX will not generate an entry for it in the ioc file
    // so we search for a I2C<i>_SDA signal set on a GPIO
    for v in config.values() {
        if let Some(signal) = v.get("Signal") {
            if let Some(captures) = re.captures(signal) {
                let name_upper = String::from(captures.get(1).unwrap().as_str());
                let name_lower = name_upper.to_ascii_lowercase();
                let mut mode = None;

                if let Some(i2c_params) = config.get::<str>(&name_upper) {
                    mode = parse_optional_param(i2c_params, "I2C_Speed_Mode")?;
                }

                i2cs.push(I2C {
                    name_lower,
                    name_upper,
                    mode,
                });
            }
        };
    }

    Ok(i2cs)
}

parameter!(
    Mode,
    [I2C_Standard, I2C_Fast, I2C_Fast_Plus],
    default = I2C_Standard
);
