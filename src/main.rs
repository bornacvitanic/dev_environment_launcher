use std::fs;
use std::path::Path;
use std::str::FromStr;
use std::fmt;
use structopt::StructOpt;
use crate::cli::Cli;

mod cli;
mod utils;

#[derive(Debug)]
enum ProjectType {
    Unity,
    Rust
}

impl FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unity" => Ok(ProjectType::Unity),
            "rust" => Ok(ProjectType::Rust),
            _ => Err(format!("'{}' is not a valid project type", s)),
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Unity => write!(f, "unity"),
            ProjectType::Rust => write!(f, "rust"),
        }
    }
}

fn main() {
    let args = Cli::from_args();
    println!("Project type: {:?}", args.project_type);
    println!("Project directory: {:?}", args.project_dir);

    if args.project_dir.is_dir() {
        match args.project_type {
            ProjectType::Unity => { open_sln_file(&args.project_dir); }
            ProjectType::Rust => { open_rust_project(&args.project_dir); }
        }
    } else {
        eprintln!("Provided path is not a directory.")
    }
}

fn open_sln_file(project_path: &Path){
    for entry in fs::read_dir(project_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().unwrap_or_default() == "sln" {
            println!("Opening solution file: {}", path.display());
            utils::open_file(&path);
            utils::open_lazygit(&project_path);
            return;
        }
    }
    eprintln!("No .sln file found in the project directory.")
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