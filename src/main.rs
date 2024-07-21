use std::{env, fs};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::cli::Cli;
use crate::config::Config;
use crate::project_type::ProjectType;
use crate::rust::open_rust_project;
use crate::unity::open_unity_project;
use crate::utils::prompt_user_for_path;

mod cli;
mod utils;
mod project_type;
mod unity;
mod rust;
mod config;

fn main() {
    let app_name = "dev_environment_launcher";
    let config_dir = match env::consts::OS {
        "windows" => {
            PathBuf::from(env::var("APPDATA").unwrap()).join(app_name)
        },
        "macos" => {
            PathBuf::from(env::var("HOME").unwrap()).join("Library/Application Support").join(app_name)
        },
        "linux" => {
            PathBuf::from(env::var("HOME").unwrap()).join(".config").join(app_name)
        },
        _ => panic!("Unsupported OS"),
    };

    // Create the full path to the configuration file
    let config_path = config_dir.join("config.toml");

    // Ensure the configuration directory exists
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    fs::create_dir_all(config_dir).expect("Failed to create config directory");

    if !std::path::Path::new(&config_path).exists() {
        Config::create_default(&config_path).expect("Failed to create default configuration file.");
        println!("Created default configuration file at {}", config_path.display());
    }

    let mut config = Config::from_file(&config_path).expect("Failed to load configuration");

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
        Some(ProjectType::Unity) => {
            if config.unity.editor_base_path.to_str() == Some("") {
                let path = prompt_user_for_path("Enter the Unity editor base path: ");
                config.unity.editor_base_path = path;
                config.save_to_file(&config_path).expect("Failed to save configuration.");
            }
            open_unity_project(config.unity.editor_base_path, &project_dir);
        }
        Some(ProjectType::Rust) => {
            if config.rust.ide_path.to_str() == Some("") {
                let path = prompt_user_for_path("Enter the Rust IDE path: ");
                config.rust.ide_path = path;
                config.save_to_file(&config_path).expect("Failed to save configuration.");
            }
            open_rust_project(&config.rust.ide_path, &project_dir);
        }
        None => eprintln!("Project type not recognized."),
    }
}