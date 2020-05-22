use phf::{phf_map, Map};

pub static AF_MAP: Map<&str, &Map<&str, u8>> = phf_map! {
    "I2C1_SCL" => &I2C1_SCL,
    "I2C1_SDA" => &I2C1_SDA,
    "I2C2_SCL" => &I2C2_SCL,
    "I2C2_SDA" => &I2C2_SDA,
    "SPI1_MISO" => &SPI1_MISO,
    "SPI1_MOSI" => &SPI1_MOSI,
    "SPI1_SCK" => &SPI1_SCK,
    "SPI2_MISO" => &SPI2_MISO,
    "SPI2_MOSI" => &SPI2_MOSI,
    "SPI2_SCK" => &SPI2_SCK,
    "USART1_RX" => &USART1_RX,
    "USART1_TX" => &USART1_TX,
    "USART2_RX" => &USART2_RX,
    "USART2_TX" => &USART2_TX,
    "USART3_RX" => &USART3_RX,
    "USART3_TX" => &USART3_TX,
    "USART4_RX" => &USART4_RX,
    "USART4_TX" => &USART4_TX,
    "USART5_RX" => &USART5_RX,
    "USART5_TX" => &USART5_TX,
    "USART6_RX" => &USART6_RX,
    "USART6_TX" => &USART6_TX,
    "USART7_RX" => &USART7_RX,
    "USART7_TX" => &USART7_TX,
    "USART8_RX" => &USART8_RX,
    "USART8_TX" => &USART8_TX,
};

static I2C1_SCL: Map<&str, u8> = phf_map! {
    "pa9" => 4,
    "pa11" => 5,
    "pb6" => 1,
    "pb8" => 1,
    "pb10" => 1,
    "pb13" => 5,
    "pf1" => 1,
};

static I2C1_SDA: Map<&str, u8> = phf_map! {
    "pa10" => 4,
    "pa12" => 5,
    "pb7" => 1,
    "pb9" => 1,
    "pb11" => 1,
    "pb14" => 5,
    "pf0" => 1,
};

static I2C2_SCL: Map<&str, u8> = phf_map! {
    "pa11" => 5,
    "pb10" => 1,
    "pb13" => 5,
};

static I2C2_SDA: Map<&str, u8> = phf_map! {
    "pa12" => 5,
    "pb11" => 1,
    "pb14" => 5,
};

static SPI1_MISO: Map<&str, u8> = phf_map! {
    "pa6" => 0,
    "pb4" => 0,
    "pb14" => 0,
    "pe14" => 1,
};

static SPI1_MOSI: Map<&str, u8> = phf_map! {
    "pa7" => 0,
    "pb5" => 0,
    "pb15" => 0,
    "pe15" => 1,
};

static SPI1_SCK: Map<&str, u8> = phf_map! {
    "pa5" => 0,
    "pb3" => 0,
    "pb13" => 0,
    "pe13" => 1,
};

static SPI2_MISO: Map<&str, u8> = phf_map! {
    "pb14" => 0,
    "pc2" => 1,
    "pd3" => 1,
};

static SPI2_MOSI: Map<&str, u8> = phf_map! {
    "pb15" => 0,
    "pc3" => 1,
    "pd4" => 1,
};

static SPI2_SCK: Map<&str, u8> = phf_map! {
    "pb10" => 5,
    "pb13" => 0,
    "pd1" => 1,
};

static USART1_RX: Map<&str, u8> = phf_map! {
    "pa3" => 1,
    "pa10" => 1,
    "pa15" => 1,
    "pb7" => 0,
};

static USART1_TX: Map<&str, u8> = phf_map! {
    "pa2" => 1,
    "pa9" => 1,
    "pa14" => 1,
    "pb6" => 0,
};

static USART2_RX: Map<&str, u8> = phf_map! {
    "pa3" => 1,
    "pa15" => 1,
    "pd6" => 0,
};

static USART2_TX: Map<&str, u8> = phf_map! {
    "pa2" => 1,
    "pa14" => 1,
    "pd5" => 0,
};

static USART3_RX: Map<&str, u8> = phf_map! {
    "pb11" => 4,
    "pc5" => 1,
    "pc11" => 1,
    "pd9" => 0,
};

static USART3_TX: Map<&str, u8> = phf_map! {
    "pb10" => 4,
    "pc4" => 1,
    "pc10" => 1,
    "pd8" => 0,
};

static USART4_RX: Map<&str, u8> = phf_map! {
    "pa1" => 4,
    "pc11" => 0,
    "pe9" => 1,
};

static USART4_TX: Map<&str, u8> = phf_map! {
    "pa0" => 4,
    "pc10" => 0,
    "pe8" => 1,
};

static USART5_RX: Map<&str, u8> = phf_map! {
    "pb4" => 4,
    "pd2" => 2,
    "pe11" => 1,
};

static USART5_TX: Map<&str, u8> = phf_map! {
    "pb3" => 4,
    "pc12" => 2,
    "pe10" => 1,
};

static USART6_RX: Map<&str, u8> = phf_map! {
    "pa5" => 5,
    "pc1" => 2,
    "pf10" => 1,
};

static USART6_TX: Map<&str, u8> = phf_map! {
    "pa4" => 5,
    "pc0" => 2,
    "pf9" => 1,
};

static USART7_RX: Map<&str, u8> = phf_map! {
    "pc1" => 1,
    "pc7" => 1,
    "pf3" => 1,
};

static USART7_TX: Map<&str, u8> = phf_map! {
    "pc0" => 1,
    "pc6" => 1,
    "pf2" => 1,
};

static USART8_RX: Map<&str, u8> = phf_map! {
    "pc3" => 2,
    "pc9" => 1,
    "pd14" => 0,
};

static USART8_TX: Map<&str, u8> = phf_map! {
    "pc2" => 2,
    "pc8" => 1,
    "pd13" => 0,
};
