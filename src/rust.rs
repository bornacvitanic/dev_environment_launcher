use std::path::Path;
use std::process::Command;
use crate::utils;

pub fn open_rust_project(ide_path: &Path, project_path: &Path){
    if project_path.exists() && project_path.is_dir() {
        println!("Opening Rust project: {}", project_path.display());
        open_in_ide(&ide_path, &project_path);
        utils::open_lazygit(&project_path);
    }else {
        eprintln!("No project directory provided.");
    }
}

pub fn open_in_ide(ide_path: &Path, project_path: &Path) {
    if !project_path.exists() {
        eprintln!("Directory does not exist: {}", project_path.display());
        return;
    }

    Command::new(ide_path)
        .arg(project_path)
        .spawn()
        .unwrap();
}