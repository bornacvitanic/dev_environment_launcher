use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::{utils};

pub fn open_unity_project(unity_hub_path: PathBuf, project_path: &Path){
    open_in_unity(unity_hub_path, project_path);
    open_sln_file(project_path);
    utils::open_lazygit(project_path);
    let packages_path = project_path.join("Packages");
    utils::open_directory(&packages_path);
    let packages = get_packages(&packages_path);
    for package in &packages {
        utils::open_lazygit(package);
    }
}

pub fn get_packages(packages_path: &Path) -> Vec<PathBuf>{
    let mut packages = Vec::new();
    for entry  in fs::read_dir(packages_path).unwrap() {
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

pub fn get_unity_editor_path(unity_hub_path: PathBuf, unity_version: &str) -> PathBuf {
    unity_hub_path.join(unity_version).join("Editor").join("Unity.exe")
}

pub fn open_in_unity(unity_hub_path: PathBuf, project_path: &Path) {
    if !project_path.exists() {
        eprintln!("Project directory does not exist: {}", project_path.display());
        return;
    }

    match get_unity_version(project_path) {
        Some(unity_version) => {
            let unity_executable_path = get_unity_editor_path(unity_hub_path, &unity_version);
            let result = Command::new(unity_executable_path)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::tempdir;
    use std::io::Write;

    #[test]
    fn test_get_packages() {
        let temp_dir = tempdir().unwrap();
        let packages_path = temp_dir.path();

        // Create mock package directories with package.json files
        let package1_path = packages_path.join("package1");
        fs::create_dir(&package1_path).unwrap();
        File::create(package1_path.join("package.json")).unwrap();

        let package2_path = packages_path.join("package2");
        fs::create_dir(&package2_path).unwrap();
        File::create(package2_path.join("package.json")).unwrap();

        let package3_path = packages_path.join("package3");
        fs::create_dir(&package3_path).unwrap();
        // No package.json file in package3

        let packages = get_packages(&packages_path);
        assert_eq!(packages.len(), 2);
        assert!(packages.contains(&package1_path));
        assert!(packages.contains(&package2_path));
    }

    #[test]
    fn test_get_packages_empty() {
        let temp_dir = tempdir().unwrap();
        let packages_path = temp_dir.path();

        let packages = get_packages(&packages_path);
        assert!(packages.is_empty());
    }

    #[test]
    fn test_get_unity_version() {
        let temp_dir = tempdir().unwrap();
        let project_settings_path = temp_dir.path().join("ProjectSettings");
        fs::create_dir(&project_settings_path).unwrap();
        let version_file_path = project_settings_path.join("ProjectVersion.txt");

        // Write a mock ProjectVersion.txt file
        let mut file = File::create(version_file_path).unwrap();
        writeln!(file, "m_EditorVersion: 2019.4.1f1").unwrap();

        let version = get_unity_version(temp_dir.path()).unwrap();
        assert_eq!(version, "2019.4.1f1");
    }

    #[test]
    fn test_get_unity_version_no_file() {
        let temp_dir = tempdir().unwrap();

        let version = get_unity_version(temp_dir.path());
        assert!(version.is_none());
    }

    #[test]
    fn test_get_unity_version_invalid_format() {
        let temp_dir = tempdir().unwrap();
        let project_settings_path = temp_dir.path().join("ProjectSettings");
        fs::create_dir(&project_settings_path).unwrap();
        let version_file_path = project_settings_path.join("ProjectVersion.txt");

        // Write a mock ProjectVersion.txt file with invalid format
        let mut file = File::create(version_file_path).unwrap();
        writeln!(file, "some_invalid_format").unwrap();

        let version = get_unity_version(temp_dir.path());
        assert!(version.is_none());
    }
}