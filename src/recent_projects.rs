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

    pub fn remove_project(&mut self, index: usize) -> Option<PathBuf> {
        if index < self.projects.len() {
            Some(self.projects.remove(index))
        } else {
            None
        }
    }

    pub fn clear_projects(&mut self) {
        self.projects.clear()
    }

    pub fn get_project(&self, index: usize) -> Option<&PathBuf> {
        self.projects.get(index)
    }

    pub fn list_projects(&self) {
        for (index, project) in self.projects.iter().enumerate() {
            println!("{}: {}", index, project.display());
        }
    }

    fn format_project_display(name: &str, path: &str, max_name_length: usize) -> String {
        // Format the project name to be left-aligned and padded to the maximum length
        let formatted_name = format!("{:<width$}", name, width = max_name_length);

        // Combine the formatted name and path with a separator (e.g., tabs or spaces)
        format!("{}    {}", formatted_name, path)
    }

    pub fn interactive_menu(&self) -> Option<PathBuf> {
        if self.projects.is_empty() {
            println!("No recent projects available.");
            return None;
        }

        // Calculate the maximum length of the project names
        let max_name_length = self.projects.iter().map(|p| {
            p.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("Unknown file")
                .len()
        }).max().unwrap_or(0);

        let items: Vec<String> = self.projects.iter().map(|p| {
            let file_name = p.file_name()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("Unknown file");
            let parent = p.parent()
                .and_then(|os_str| os_str.to_str())
                .unwrap_or("Unknown parent");

            Self::format_project_display(file_name, parent, max_name_length)
        }).collect();

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Select a recent project")
            .default(0)
            .items(&items)
            .interact()
            .ok()?;

        self.get_project(selection).cloned()
    }
}