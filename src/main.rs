use crate::cli::{Cli, Command};
use crate::config::Config;
use crate::project_type::ProjectType;
use crate::recent_projects::RecentProjects;
use crate::rust::open_rust_project;
use crate::unity::open_unity_project;
use crate::utils::prompt_user_for_path;
use std::env;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

mod cli;
mod config;
mod project_type;
mod recent_projects;
mod rust;
mod unity;
mod utils;

const APP_NAME: &str = "dev_environment_launcher";

fn main() {
    let config_dir = Config::get_config_dir(APP_NAME);
    let config_path = config_dir.join("config.toml");

    let mut config = match Config::get_config(&config_dir, &config_path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    let args = Cli::from_args();
    let mut recent_projects = match RecentProjects::load(&config_dir) {
        Ok(recent_projects) => recent_projects,
        Err(e) => {
            eprintln!("Failed to load recent projects: {}", e);
            return;
        }
    };

    match args.command {
        Some(Command::Path { path }) => open_project(
            path,
            &mut config,
            &config_path,
            &mut recent_projects,
            &config_dir,
        ),
        Some(Command::Open { index }) => open_recent_project(
            index,
            &mut recent_projects,
            &mut config,
            &config_path,
            &config_dir,
        ),
        Some(Command::Remove { index }) => remove_project(index, &mut recent_projects, &config_dir),
        Some(Command::Clear) => clear_recent_projects(&mut recent_projects, &config_dir),
        Some(Command::Options) => recent_projects.list_projects(),
        Some(Command::Recent) => {
            open_interactive_project(&mut recent_projects, &mut config, &config_path, &config_dir)
        }
        None => {
            open_current_directory(&mut config, &config_path, &mut recent_projects, &config_dir)
        }
    }
}

fn open_project(
    project_dir: PathBuf,
    config: &mut Config,
    config_path: &Path,
    recent_projects: &mut RecentProjects,
    config_dir: &Path,
) {
    if !project_dir.is_dir() {
        eprintln!("Provided path is not a directory.");
        return;
    }

    match ProjectType::from_path(&project_dir) {
        Some(project_type) => {
            println!("Project type: {:?}", &project_type);
            match project_type {
                ProjectType::Unity => open_unity(config, config_path, &project_dir),
                ProjectType::Rust => open_rust(config, config_path, &project_dir),
            }
            recent_projects.add_project(project_dir);
            save_recent_projects(config_dir, recent_projects);
        }
        None => eprintln!("Project type not recognized."),
    }
}

fn open_unity(config: &mut Config, config_path: &Path, project_dir: &Path) {
    if config
        .unity
        .editor_base_path
        .to_str()
        .unwrap_or("")
        .is_empty()
    {
        config.unity.editor_base_path = prompt_user_for_path("Enter the Unity editor base path: ");
        save_config(config_path, config);
    }
    if config
        .unity
        .json_editor_path
        .to_str()
        .unwrap_or("")
        .is_empty()
    {
        config.unity.editor_base_path = prompt_user_for_path("Enter the json editor base path: ");
        save_config(config_path, config);
    }
    open_unity_project(config.unity.editor_base_path.clone(), project_dir, &config.unity.json_editor_path);
}

fn open_rust(config: &mut Config, config_path: &Path, project_dir: &Path) {
    if config.rust.ide_path.to_str().unwrap_or("").is_empty() {
        config.rust.ide_path = prompt_user_for_path("Enter the Rust IDE path: ");
        save_config(config_path, config);
    }
    open_rust_project(&config.rust.ide_path, project_dir);
}

fn open_recent_project(
    index: usize,
    recent_projects: &mut RecentProjects,
    config: &mut Config,
    config_path: &Path,
    config_dir: &Path,
) {
    if let Some(project) = recent_projects.get_project(index) {
        open_project(
            project.clone(),
            config,
            config_path,
            recent_projects,
            config_dir,
        );
    } else {
        eprintln!("Invalid recent project index.");
    }
}

fn remove_project(index: usize, recent_projects: &mut RecentProjects, config_dir: &Path) {
    if let Some(project) = recent_projects.remove_project(index) {
        println!("Removed {} from recent projects", project.display());
        save_recent_projects(config_dir, recent_projects);
    } else {
        eprintln!("Invalid recent project index.");
    }
}

fn clear_recent_projects(recent_projects: &mut RecentProjects, config_dir: &Path) {
    recent_projects.clear_projects();
    save_recent_projects(config_dir, recent_projects);
    println!("Cleared all recent projects.");
}

fn open_interactive_project(
    recent_projects: &mut RecentProjects,
    config: &mut Config,
    config_path: &Path,
    config_dir: &Path,
) {
    if let Some(project) = recent_projects.interactive_menu() {
        open_project(project, config, config_path, recent_projects, config_dir);
    }
}

fn open_current_directory(
    config: &mut Config,
    config_path: &Path,
    recent_projects: &mut RecentProjects,
    config_dir: &Path,
) {
    let project_dir = env::current_dir().expect("Failed to get current directory");
    open_project(
        project_dir,
        config,
        config_path,
        recent_projects,
        config_dir,
    );
}

fn save_config(config_path: &Path, config: &mut Config) {
    config
        .save_to_file(config_path)
        .expect("Failed to save configuration.");
}

fn save_recent_projects(config_dir: &Path, recent_projects: &mut RecentProjects) {
    recent_projects
        .save(config_dir)
        .expect("Failed to save recent projects.");
}