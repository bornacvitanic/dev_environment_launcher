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