use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs;
use dialoguer::Select;
use dialoguer::theme::{ColorfulTheme};
use serde_derive::{Deserialize, Serialize};

const RECENT_PROJECTS_FILE: &str = "recent_projects.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct RecentProjects {
    projects: Vec<PathBuf>
}

impl RecentProjects {
    pub fn load(config_dir: &Path) -> Result<Self, Box<dyn Error>> {
        let path = config_dir.join(RECENT_PROJECTS_FILE);
        if path.exists() {
            let data = fs::read_to_string(path)?;
            let projects: RecentProjects = toml::from_str(&data)?;
            Ok(projects)
        } else {
            Ok(RecentProjects { projects: Vec::new() })
        }
    }

    pub fn save(&self, config_dir: &Path) -> Result<(), Box<dyn Error>> {
        let path = config_dir.join(RECENT_PROJECTS_FILE);
        let data = toml::to_string_pretty(&self)?;
        fs::write(path, data)?;
        Ok(())
    }

    pub fn add_project(&mut self, project: PathBuf) {
        if self.projects.contains(&project) {
            return;
        }
        self.projects.push(project);
        if self.projects.len() > 10 {
            self.projects.remove(0);
        }
    }

    pub fn get_project(&self, index: usize) -> Option<&PathBuf> {
        self.projects.get(index)
    }

    pub fn list_projects(&self) {
        for (index, project) in self.projects.iter().enumerate() {
            println!("{}: {}", index, project.display());
        }
    }

    pub fn interactive_menu(&self) -> Option<PathBuf> {
        if self.projects.is_empty() {
            println!("No recent projects available.");
            return None;
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a recent project")
            .default(0)
            .items(&self.projects.iter().map(|p| p.to_str().unwrap()).collect::<Vec<_>>())
            .interact()
            .ok()?;

        self.get_project(selection).cloned()
    }
}