#![warn(rust_2018_idioms)]

#[macro_use]
extern crate fstrings;

#[macro_use]
mod utils;
mod db;
mod generate;
mod gpio;
mod rcc;
mod spi;

use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, bail, ensure, Context};

use crate::gpio::GpioPin;
use crate::rcc::RCC;
use crate::spi::SPI;
use crate::utils::*;
use std::fs::OpenOptions;
use std::process::Command;

type ConfigParams<'a> = HashMap<&'a str, HashMap<&'a str, &'a str>>;

#[derive(Debug)]
pub struct Config {
    pub version: String,
    pub mcu_family: MCUFamily,
    pub mcu_name: String,
    pub rcc: RCC,
    pub gpios: Vec<GpioPin>,
    pub ports: Vec<char>,
    pub spis: Vec<SPI>,
}

pub fn load_ioc(file_content: &str) -> anyhow::Result<Config> {
    let config_params = parse_ioc(file_content);

    let version = String::from(
        *config_params
            .get("File")
            .ok_or_else(|| anyhow!("Couldn't check ioc version"))?
            .get("Version")
            .ok_or_else(|| anyhow!("Couldn't check ioc version"))?,
    );

    let mcu = config_params
        .get("Mcu")
        .ok_or_else(|| anyhow!("Couldn't check MCU information"))?;

    let mcu_family = parse_mandatory_param(mcu, "Family")?;

    let mcu_name = mcu
        .get("UserName")
        .ok_or_else(|| anyhow!("Couldn't check MCU name"))?
        .to_string();

    let rcc = rcc::get_rcc(&config_params).context("Parsing of RCC")?;

    let (ports, gpios) = gpio::get_gpios(&config_params).context("Parsing of GPIOs")?;

    let spis = spi::get_spis(&config_params).context("Parsing of SPIs")?;

    Ok(Config {
        version,
        mcu_family,
        mcu_name,
        rcc,
        gpios,
        ports,
        spis,
    })
}

pub fn parse_ioc(file_content: &str) -> ConfigParams<'_> {
    let mut config_params = HashMap::new();

    for line in file_content.lines() {
        let name_and_value: Vec<&str> = line.split('=').collect();

        if let [name, value] = name_and_value[..] {
            let object_and_parameter: Vec<&str> = name.split('.').collect();
            if let [object_name, parameter_name] = object_and_parameter[..] {
                config_params
                    .entry(object_name)
                    .or_insert_with(HashMap::new)
                    .insert(parameter_name, value);
            }
        }
    }

    config_params
}

fn cargo_init(project_dir: &Path) -> anyhow::Result<bool> {
    let output = if project_dir.eq(Path::new("")) {
        // empty path as current_dir doesn't work, not sure why
        Command::new("cargo").arg("init").output()
    } else {
        Command::new("cargo")
            .arg("init")
            .current_dir(project_dir)
            .output()
    }
    .context("cargo init")?;

    let output = String::from_utf8(output.stderr).unwrap();
    Ok(output.contains("Created binary (application) package"))
}

pub fn generate(project_dir: &Path, config: Config) -> anyhow::Result<()> {
    ensure!(
        config.version == "6",
        "only File.Version=6 supported in ioc file"
    );

    // run cargo init
    let package_created = cargo_init(project_dir)?;

    // append to Cargo.toml
    // TODO replace this with calls to cargo add, once cargo #5586 is through
    if package_created {
        println!("Ran cargo init");
        let cargo_toml = project_dir.join("Cargo.toml");
        let mut file = OpenOptions::new().append(true).open(cargo_toml)?;

        let dependencies = generate::generate_dependencies(&config)?;
        write!(file, "{}", dependencies)?;
        println!("Added dependencies to Cargo.toml");
    } else {
        println!("Detected existing project");
    }

    // src/main.rs
    let main_rs = generate::generate_main(&config)?;
    println!("Generated src/main.rs");

    let path_to_main = project_dir.join("src/main.rs");
    fs::write(path_to_main, main_rs).context("write to main.rs")?;

    // .cargo/config
    let cargo_config = generate::generate_cargo_config(&config);

    let path_to_cargo_cofig = project_dir.join(".cargo/config");
    fs::create_dir_all(path_to_cargo_cofig.parent().unwrap()).unwrap();
    fs::write(path_to_cargo_cofig, cargo_config).context("write to config")?;
    println!("Generated .cargo/config");

    // memory.x
    let memory_config = generate::generate_memory_x(&config)?;

    let path_to_memory_x = project_dir.join("memory.x");
    fs::write(path_to_memory_x, memory_config).context("write to memory.x")?;
    println!("Generated memory.x");

    Ok(())
}

parameter!(
    MCUFamily,
    [
        STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32G0, STM32G4, STM32H7, STM32L0,
        STM32L1, STM32L4, STM32L5, STM32MP1, STM32WB, STM32WL
    ]
);
