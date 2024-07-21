use std::{env};
use structopt::StructOpt;
use crate::cli::Cli;
use crate::project_type::ProjectType;
use crate::rust::open_rust_project;
use crate::unity::open_unity_project;

mod cli;
mod utils;
mod project_type;
mod unity;
mod rust;

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