use std::fs;
use std::path::{Path, PathBuf};
use crate::{utils};

pub fn open_unity_project(project_path: &Path){
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

pub fn get_packages(packages_path: &Path) -> Vec<PathBuf>{
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

pub fn open_sln_file(project_path: &Path){
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