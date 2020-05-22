use std::path::{Path, PathBuf};
use std::{env, fs};

use anyhow::anyhow;

fn get_path_to_ioc_file(dir: &Path) -> anyhow::Result<PathBuf> {
    let mut path_to_ioc_file: Option<PathBuf> = None;

    for entry in dir.read_dir()? {
        let entry = entry?;
        match entry.file_name().to_str() {
            None => {}
            Some(filename) => {
                if filename.ends_with(".ioc") {
                    if path_to_ioc_file.is_none() {
                        path_to_ioc_file = Some(entry.path());
                    } else {
                        return Err(anyhow!("More than one .ioc file"));
                    }
                }
            }
        }
    }

    path_to_ioc_file.ok_or_else(|| anyhow!("No .ioc file fond"))
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let default_path = String::new();
    let project_dir = Path::new(args.get(1).unwrap_or(&default_path));

    let path_to_ioc_file: PathBuf = get_path_to_ioc_file(project_dir)?;
    
    println!("Found ioc file {:?}", path_to_ioc_file.file_name().unwrap());
    let filecontent = fs::read_to_string(path_to_ioc_file)?;
    
    let config = cube2rust::load_ioc(&filecontent)?;
    println!("Loaded ioc file");

    cube2rust::generate(project_dir, config)
}
