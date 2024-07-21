use std::{env, fs};
use std::path::{Path, PathBuf};
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

    let project_dir = args.project_dir
        .unwrap_or_else(|| env::current_dir().expect("Failed to get current directory"));

    println!("Project directory: {:?}", project_dir);

    if project_dir.is_dir() {
        match args.project_type {
            ProjectType::Unity => open_unity_project(&project_dir),
            ProjectType::Rust => open_rust_project(&project_dir),
        }
    } else {
        eprintln!("Provided path is not a directory.");
    }
}

fn open_unity_project(project_path: &Path){
    utils::open_unity_project(&project_path);
    open_sln_file(&project_path);
    utils::open_lazygit(&project_path);
    let packages_path = project_path.join("Packages");
    utils::open_directory(&packages_path);
    let packages = get_packages(&packages_path);
    for package in &packages {
        utils::open_lazygit(&package);
    }
}

fn get_packages(packages_path: &Path) -> Vec<PathBuf>{
    let mut packages = Vec::new();
    for entry  in fs::read_dir(&packages_path).unwrap() {
        match entry {
            Ok(entry) => {
                let package_path = entry.path();
                let package_json = package_path.join("package.json");
                if package_path.is_dir() && package_json.exists() {
                    packages.push(package_path);
                }
            }
            Err(e) => eprintln!("Error reading package entry: {}, Error: {}", packages_path.display(), e)
        }
    }
    packages
}

fn open_sln_file(project_path: &Path){
    for entry in fs::read_dir(project_path).unwrap() {
        let entry = entry.unwrap();
        let file_path = entry.path();
        if file_path.extension().unwrap_or_default() == "sln" {
            println!("Opening solution file: {}", file_path.display());
            utils::open_file(&file_path);
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