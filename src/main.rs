use std::{env, fs};
use structopt::StructOpt;
use crate::cli::{Cli, Command};
use crate::config::Config;
use crate::project_type::ProjectType;
use crate::recent_projects::RecentProjects;
use crate::rust::open_rust_project;
use crate::unity::open_unity_project;
use crate::utils::prompt_user_for_path;

mod cli;
mod utils;
mod project_type;
mod unity;
mod rust;
mod config;
mod recent_projects;

const APP_NAME: &str = "dev_environment_launcher";

fn main() {
    let config_dir = Config::get_config_dir(APP_NAME);

    // Create the full path to the configuration file
    let config_path = config_dir.join("config.toml");

    // Ensure the configuration directory exists
    fs::create_dir_all(&config_dir).expect("Failed to create config directory");

    if !std::path::Path::new(&config_path).exists() {
        Config::create_default(&config_path).expect("Failed to create default configuration file.");
        println!("Created default configuration file at {}", config_path.display());
    }

    let mut config = Config::from_file(&config_path).expect("Failed to load configuration");

    let args = Cli::from_args();

    let mut recent_projects = RecentProjects::load(&config_dir).expect("Failed to load recent projects");

    let project_dir = match args.command {
        Some(Command::Path { path }) => path,
        Some(Command::Recent { index }) => {
            if let Some(project) = recent_projects.get_project(index) {
                project.clone()
            } else {
                eprintln!("Invalid recent project index.");
                return;
            }
        }
        Some(Command::Remove { index}) => {
            if let Some(project) = recent_projects.remove_project(index){
                println!("Removed {} from recent projects", project.display());
                recent_projects.save(&config_dir).expect("Failed to save recent projects.");
            } else {
                eprintln!("Invalid recent project index.")
            }
            return;
        }
        Some(Command::List) => {
            recent_projects.list_projects();
            return;
        }
        Some(Command::Menu) => {
            if let Some(project) = recent_projects.interactive_menu() {
                project
            } else {
                return;
            }
        }
        None => env::current_dir().expect("Failed to get current directory")
    };

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

    recent_projects.add_project(project_dir.clone());
    recent_projects.save(&config_dir).expect("Failed to save recent projects.")
}