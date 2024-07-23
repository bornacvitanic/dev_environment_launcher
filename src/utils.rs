use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn prompt_user_for_path(prompt: &str) -> PathBuf {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    PathBuf::from(input.trim())
}

pub fn open_file(path: &Path) {
    if !path.exists() {
        eprintln!("File does not exist: {}", path.display());
        return;
    }

    #[cfg(target_os = "windows")]
    {
        let result = Command::new("cmd")
            .args(["/C", "start", "", path.to_str().unwrap()])
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
        let result = Command::new("explorer").arg(path).spawn();

        match result {
            Ok(_) => println!("Opened directory {}", path.display()),
            Err(e) => eprintln!("Error opening directory: {}, Error: {}", path.display(), e),
        }
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
            .args([
                "/C",
                "start",
                "cmd",
                "/K",
                "lazygit",
                "-p",
                git_path.to_str().unwrap(),
            ])
            .spawn()
            .unwrap();
    }
}
