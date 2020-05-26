use crate::*;

const MAX_USARTS: u8 = 8;

#[derive(Debug)]
pub struct USART {
    pub name_lower: String,
    pub name_upper: String,
    pub baudrate: Option<u32>,
}

pub fn get_usarts(config: &ConfigParams<'_>) -> anyhow::Result<Vec<USART>> {
    let mut usarts = Vec::new();

    for i in 1..=MAX_USARTS {
        let name_lower = f!("usart{i}");
        let name_upper = name_lower.to_ascii_uppercase();

        // search config for USART1, USART2, etc..
        if let Some(usart_params) = config.get::<str>(&name_upper) {
            let baudrate = parse_optional_u32(usart_params, "BaudRate")?;

            usarts.push(USART {
                name_lower,
                name_upper,
                baudrate,
            });
        }
    }
    Ok(usarts)
}
