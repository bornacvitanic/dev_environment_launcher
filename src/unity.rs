use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::{utils};

pub fn open_unity_project(project_path: &Path){
    open_in_unity(&project_path);
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

pub fn get_unity_version(project_path: &Path) -> Option<String> {
    let version_file_path = project_path.join("ProjectSettings").join("ProjectVersion.txt");
    if let Ok(contents) = fs::read_to_string(version_file_path) {
        for line in contents.lines() {
            if line.starts_with("m_EditorVersion:") {
                let version = line.split_whitespace().nth(1)?.to_string();
                return Some(version);
            }
        }
    }
    None
}

pub fn get_unity_editor_path(unity_version: &str) -> String {
    format!("C:\\Program Files\\Unity\\Hub\\Editor\\{}\\Editor\\Unity.exe", unity_version)
}

pub fn open_in_unity(project_path: &Path) {
    if !project_path.exists() {
        eprintln!("Project directory does not exist: {}", project_path.display());
        return;
    }

    match get_unity_version(project_path) {
        Some(unity_version) => {
            let unity_executable_path = get_unity_editor_path(&unity_version);
            let result = Command::new(&unity_executable_path)
                .arg("-projectPath")
                .arg(project_path)
                .spawn();

            match result {
                Ok(_) => println!("Opened Unity project with version {}: {}", unity_version, project_path.display()),
                Err(e) => eprintln!("Failed to open Unity project: {}. Error: {}", project_path.display(), e),
            }
        },
        None => eprintln!("Failed to read Unity version from ProjectVersion.txt"),
    }
}