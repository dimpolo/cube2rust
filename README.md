# cube2rust
A tool for generating a rust project from a STM32CubeMX ioc file.

The tool will run `cargo init` in the same directory as the ioc file.

It will then add dependencies to `Cargo.toml` and generate a `src/main.rs`, `.cargo/config` and `memory.x`.

Currently, running this tool will overwrite everything, so use with caution. 

### Installation
```bash
$ cargo install --git https://github.com/dimpolo/cube2rust.git
```
### Usage
From inside a directory containing an ioc file
```bash
$ cube2rust
```

From anywhere
```bash
$ cube2rust path/to/project_directory
```

### Currently supported
* Only STM32F0
* GPIO, RCC, SPI, USART
