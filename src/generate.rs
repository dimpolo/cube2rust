use crate::db::*;
use crate::gpio::*;
use crate::rcc::*;
use crate::spi::*;
use crate::utils::*;
use crate::{Config, MCUFamily};

pub fn generate_main(config: &Config) -> anyhow::Result<String> {
    let hal = match config.mcu_family {
        MCUFamily::STM32F0 => "stm32f0xx_hal",
        _ => todo!("Only STM32F0 supported for now"),
    };

    let mut imports = GeneratedString::new();

    imports.line("#![no_std]");
    imports.line("#![no_main]");
    imports.empty_line();
    imports.line("use crate::hal::{prelude::*, stm32};");
    imports.line("use cortex_m::interrupt;");
    imports.line("use cortex_m_rt::entry;");
    imports.line("use panic_halt as _;");
    imports.line(f!("use {hal} as hal;"));
    imports.empty_line();

    let mut main_func = GeneratedString::new();

    main_func.line("#[entry]");
    main_func.line("fn main() -> ! {");
    main_func.indent_right();

    main_func.line("let mut p = stm32::Peripherals::take().unwrap();");

    add_rcc(&mut main_func, &config);

    add_ports(&mut main_func, &config);

    add_gpios(&mut main_func, &config);

    for spi in config.spis.iter() {
        add_spi(&mut main_func, &mut imports, spi);
    }

    main_func.line("loop {}");

    main_func.indent_left();
    main_func.line("}");

    Ok(imports.string + "\n" + &main_func.string)
}

fn add_rcc(string: &mut GeneratedString, config: &Config) {
    string.line("let mut rcc = p");
    string.indent_right();
    string.line(".RCC");
    string.line(".configure()");
    match config.rcc.clock_source {
        ClockSource::HSI => {}
        ClockSource::HSI48 => string.line(".hsi48()"),
        ClockSource::HSE(HSEMode::NotBypassed(freq)) => string.line(f!(
            ".hse({freq}.hz(), hal::rcc::HSEBypassMode::NotBypassed)"
        )),
        ClockSource::HSE(HSEMode::Bypassed(freq)) => {
            string.line(f!(".hse({freq}.hz(), hal::rcc::HSEBypassMode::Bypassed)"))
        }
    }

    if let Some(sysclk_freq) = config.rcc.sysclk_freq {
        string.line(f!(".sysclk({sysclk_freq}.hz())"));
    }
    if let Some(hclk_freq) = config.rcc.hclk_freq {
        string.line(f!(".hclk({hclk_freq}.hz())"));
    }
    if let Some(apb1_freq) = config.rcc.apb1_freq {
        string.line(f!(".pclk({apb1_freq}.hz())"));
    }
    string.line(".freeze(&mut p.FLASH);");
    string.indent_left();
    string.empty_line();
}

fn add_ports(string: &mut GeneratedString, config: &Config) {
    for port in config.ports.iter() {
        let port_lower = port.to_ascii_lowercase();

        let gpio_names: Vec<_> = config
            .gpios
            .iter()
            .filter(|gpio| gpio.port.ends_with(*port))
            .map(|gpio| gpio.register.clone())
            .collect();

        let registers: Vec<_> = gpio_names
            .iter()
            .map(|name| f!("_{port_lower}.{name}"))
            .collect();

        let gpio_names = gpio_names.join(", ");
        let registers = registers.join(", ");

        string.line(f!("let _{port_lower} = p.GPIO{port}.split(&mut rcc);"));
        string.line(f!("let ({gpio_names}) = ({registers});"));
    }
    string.empty_line();
}

fn add_gpios(string: &mut GeneratedString, config: &Config) {
    for gpio in config.gpios.iter() {
        let pin_name = gpio.get_name();
        let pin_configuration = configure_gpio(gpio, config.mcu_family);

        let mutable = if !matches!(&gpio.signal, SignalType::Peripheral(_)) {
            "mut "
        } else {
            ""
        };

        string.line(f!("let {mutable}{pin_name} = {pin_configuration};"));
    }

    string.empty_line();
}

fn configure_gpio(gpio: &GpioPin, mcu_family: MCUFamily) -> String {
    let name = gpio.get_name();

    let func = match gpio.signal {
        SignalType::AdcInput => f!("into_analog"),
        SignalType::GpioInput => match gpio.pu_pd.unwrap_or_default() {
            PullType::GPIO_NOPULL => f!("into_floating_input"),
            PullType::GPIO_PULLUP => f!("into_pull_up_input"),
            PullType::GPIO_PULLDOWN => f!("into_pull_down_input"),
        },
        SignalType::GpioOutput => match gpio.mode_default_output_pp.unwrap_or_default() {
            ModeOutputType::GPIO_MODE_OUTPUT_OD => match gpio.speed.unwrap_or_default() {
                SpeedType::GPIO_SPEED_FREQ_LOW => f!("into_open_drain_output"),
                _ => todo!("{} higher speeds for", name),
            },
            ModeOutputType::GPIO_MODE_OUTPUT_PP => match gpio.speed.unwrap_or_default() {
                SpeedType::GPIO_SPEED_FREQ_LOW => f!("into_push_pull_output"),
                SpeedType::GPIO_SPEED_FREQ_MEDIUM => f!("into_push_pull_output_hs"),

                _ => todo!("{} higher speeds for", name),
            },
        },
        SignalType::Peripheral(ref name) => {
            let af = get_alternate_function(mcu_family, gpio, name);
            f!("into_alternate_af{af}")
        }
    };
    f!("interrupt::free(|cs| {gpio.register}.{func}(cs))")
}

fn add_spi(main_func: &mut GeneratedString, imports: &mut GeneratedString, spi: &SPI) {
    let polarity = match spi.polarity.unwrap_or_default() {
        CLKPolarity::SPI_POLARITY_LOW => "IdleLow",
        CLKPolarity::SPI_POLARITY_HIGH => "IdleHigh",
    };

    let phase = match spi.phase.unwrap_or_default() {
        CLKPhase::SPI_PHASE_1EDGE => "CaptureOnFirstTransition",
        CLKPhase::SPI_PHASE_2EDGE => "CaptureOnSecondTransition",
    };

    imports.line("use hal::spi::{Spi, Mode, Phase, Polarity};");

    main_func.line(f!("let mut {spi.name_lower} = Spi::{spi.name_lower}("));
    main_func.indent_right();
    main_func.line(f!("p.{spi.name_upper},"));
    main_func.line(f!(
        "({spi.name_lower}_sck, {spi.name_lower}_miso, {spi.name_lower}_mosi),"
    ));
    main_func.line("Mode {");
    main_func.indent_right();
    main_func.line(f!("polarity: Polarity::{polarity},"));
    main_func.line(f!("phase: Phase::{phase},"));
    main_func.indent_left();
    main_func.line("},");
    main_func.line(f!("{spi.baudrate.0}.hz(),"));
    main_func.line("&mut rcc");
    main_func.indent_left();
    main_func.line(");");
    main_func.empty_line();
}

pub fn generate_cargo_config(config: &Config) -> String {
    let mut file_content = String::from(
        r#"[target.'cfg(all(target_arch = "arm", target_os = "none"))']
# uncomment ONE of these three option to make `cargo run` start a GDB session
# which option to pick depends on your system
# runner = "arm-none-eabi-gdb -q -x openocd.gdb"
# runner = "gdb-multiarch -q -x openocd.gdb"
# runner = "gdb -q -x openocd.gdb"

rustflags = [
  # LLD (shipped with the Rust toolchain) is used as the default linker
  "-C", "link-arg=-Tlink.x",

  # if you run into problems with LLD switch to the GNU linker by commenting out
  # this line
  # "-C", "linker=arm-none-eabi-ld",

  # if you need to link to pre-compiled C libraries provided by a C toolchain
  # use GCC as the linker by commenting out both lines above and then
  # uncommenting the three lines below
  # "-C", "linker=arm-none-eabi-gcc",
  # "-C", "link-arg=-Wl,-Tlink.x",
  # "-C", "link-arg=-nostartfiles",
]

[build]
"#,
    );

    let target = match config.mcu_family {
        MCUFamily::STM32F0 | MCUFamily::STM32L0 | MCUFamily::STM32G0 => "thumbv6m-none-eabi",
        MCUFamily::STM32F1 | MCUFamily::STM32F2 | MCUFamily::STM32L1 => "thumbv7m-none-eabi",
        MCUFamily::STM32F3 | MCUFamily::STM32F4 => "thumbv7em-none-eabihf",
        _ => todo!("find out if it has FPU"),
    };

    file_content.push_str(&f!("target = \"{target}\"\n"));
    file_content
}

pub fn generate_dependencies(config: &Config) -> anyhow::Result<String> {
    let hal_crate = match config.mcu_family {
        MCUFamily::STM32F0 => "stm32f0xx-hal",
        _ => todo!("other hal crates"),
    };

    // TODO proper feature selection
    let feature = config.mcu_name[..9].to_ascii_lowercase();

    let mut filecontent =
        f!("{hal_crate} = {{version = \"*\", features = [\"{feature}\", \"rt\"]}}");

    filecontent.push_str(
        r#"
cortex-m = "*"
cortex-m-rt = "*"
panic-halt = "*"

[profile.dev.package."*"]
# opt-level = "z"

[profile.release]
# lto = true
"#,
    );

    Ok(filecontent)
}

pub fn generate_memory_x(config: &Config) -> anyhow::Result<String> {
    let mem_size = get_mem_size(config);

    Ok(f!("\
MEMORY
{{
  FLASH : ORIGIN = 0x00000000, LENGTH = {mem_size.flash}K
  RAM : ORIGIN = 0x20000000, LENGTH = {mem_size.ram}K
}}
"))
}
