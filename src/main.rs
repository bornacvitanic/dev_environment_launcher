use std::{env, fs};
use structopt::StructOpt;
use crate::cli::Cli;
use crate::config::Config;
use crate::project_type::ProjectType;
use crate::rust::open_rust_project;
use crate::unity::open_unity_project;

mod cli;
mod utils;
mod project_type;
mod unity;
mod rust;
mod config;

fn main() {
    let config_path = match env::consts::OS {
        "windows" => {
            format!("{}/dev_environment_launcher/config.toml", env::var("APPDATA").unwrap())
        },
        "macos" => {
            format!("{}/Library/Application Support/dev_environment_launcher/config.toml", env::var("HOME").unwrap())
        },
        "linux" => {
            format!("{}/.config/dev_environment_launcher/config.toml", env::var("HOME").unwrap())
        },
        _ => panic!("Unsupported OS"),
    };

    let config_dir = std::path::Path::new(&config_path).parent().unwrap();
    fs::create_dir_all(config_dir).expect("Failed to create config directory");

    if !std::path::Path::new(&config_path).exists() {
        Config::create_default(&config_path).expect("Failed to create default configuration file.");
        println!("Created default configuration file at {}", config_path);
    }

    let config = Config::from_file(&config_path).expect("Failed to load configuration");

    let args = Cli::from_args();

    let project_dir = args.project_dir
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    if !project_dir.is_dir() {
        eprintln!("Provided path is not a directory.");
        return;
    }

    let project_type = ProjectType::from_path(&project_dir);
    println!("Project type: {:?}", project_type);

    match project_type {
        Some(ProjectType::Unity) => open_unity_project(config.unity.editor_base_path, &project_dir),
        Some(ProjectType::Rust) => open_rust_project(config.rust.ide_path.as_path(), &project_dir),
        None => eprintln!("Project type not recognized."),
    }
}