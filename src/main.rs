use std::{env};
use std::path::{Path};
use std::str::FromStr;
use structopt::StructOpt;
use crate::cli::Cli;
use crate::project_type::ProjectType;
use crate::utils::open_unity_project;

mod cli;
mod utils;
mod project_type;
mod unity;

fn main() {
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
        Some(ProjectType::Unity) => open_unity_project(&project_dir),
        Some(ProjectType::Rust) => open_rust_project(&project_dir),
        None => eprintln!("Project type not recognized."),
    }
}

fn open_rust_project(project_path: &Path){
    if project_path.exists() && project_path.is_dir() {
        println!("Opening Rust project: {}", project_path.display());
        utils::open_in_rust_rover(&project_path);
        utils::open_lazygit(&project_path);
    }else {
        eprintln!("No project directory provided.");
    }
}