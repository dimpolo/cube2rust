use std::fs;
use std::process::{Command, Stdio};

const IOC_FILE: &str = "tests/stm32f042.ioc";

/// makes a test_project folder, copies IOC_FILE to it,
/// runs cube2rust in it and then tries to build
///
/// run this "test" with --nocapture
#[test]
fn integration_test() {
    fs::create_dir_all("../test_project/").expect("Failed to create test_project directory");
    fs::copy(IOC_FILE, "../test_project/test_project.ioc").expect("Failed to copy ioc file");

    Command::new("cargo")
        .arg("run")
        .arg("--color=always")
        .arg("--")
        .arg("../test_project")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute cube2rust");

    Command::new("cargo")
        .arg("build")
        .arg("--color=always")
        .current_dir("../test_project")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("Failed to execute cargo build");
}

/// Load in ioc file, parse it and print out parsed config
#[test]
fn test_config() {
    let filecontent = fs::read_to_string(IOC_FILE).expect("read failed");

    let config = cube2rust::load_ioc(&filecontent);

    dbg!(&config);

    assert!(config.is_ok());
}

/// Load in ioc file and print it
#[test]
fn test_params() {
    let file_content = fs::read_to_string(IOC_FILE).expect("read failed");

    let config_params = cube2rust::parse_ioc(&file_content);

    dbg!(config_params);
}
