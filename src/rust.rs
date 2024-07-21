use std::path::Path;
use std::process::Command;
use crate::utils;

pub fn open_rust_project(project_path: &Path){
    if project_path.exists() && project_path.is_dir() {
        println!("Opening Rust project: {}", project_path.display());
        open_in_rust_rover(&project_path);
        utils::open_lazygit(&project_path);
    }else {
        eprintln!("No project directory provided.");
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