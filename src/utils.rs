use std::fs;
use std::process::Command;
use std::path::Path;

pub fn open_file(path: &Path) {
    if !path.exists() {
        eprintln!("File does not exist: {}", path.display());
        return;
    }

    #[cfg(target_os = "windows")]
    {
        let result = Command::new("cmd")
            .args(&["/C", "start", "", path.to_str().unwrap()])
            .spawn();

        match result {
            Ok(_) => println!("Opened file {}", path.display()),
            Err(e) => eprintln!("Error opening file: {}, Error: {}", path.display(), e),
        }
    }
}

pub fn open_directory(path: &Path) {
    if !path.exists() {
        eprintln!("Directory does not exist: {}", path.display());
        return;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("explorer")
            .arg(path)
            .spawn()
            .unwrap();
    }
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

pub fn open_unity_project(project_path: &Path) {
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

pub fn is_git_repository(path: &Path) -> bool {
    path.join(".git").exists()
}

pub fn open_lazygit(path: &Path) {
    if !path.exists() {
        eprintln!("Directory does not exist: {}", path.display());
        return;
    }

    let git_path = if is_git_repository(path) {
        path
    } else if let Some(parent_path) = path.parent() {
        if is_git_repository(parent_path) {
            parent_path
        } else {
            eprintln!("No valid Git repository found in the specified directory or its parent.");
            return;
        }
    } else {
        eprintln!("No valid Git repository found in the specified directory or its parent.");
        return;
    };

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", "cmd", "/K", "lazygit", "-p", git_path.to_str().unwrap()])
            .spawn()
            .unwrap();
    }
}

pub fn open_in_rust_rover(path: &Path) {
    if !path.exists() {
        eprintln!("Directory does not exist: {}", path.display());
        return;
    }

    Command::new("rustrover64.exe")
        .arg(path)
        .spawn()
        .unwrap();
}