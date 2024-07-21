use std::process::Command;
use std::path::Path;

pub fn open_file(path: &Path) {
    if !path.exists() {
        eprintln!("File does not exist: {}", path.display());
        return;
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", path.to_str().unwrap()])
            .spawn()
            .unwrap();
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