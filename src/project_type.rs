use std::fmt;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
pub enum ProjectType {
    Unity,
    Rust,
}

impl FromStr for ProjectType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "unity" => Ok(ProjectType::Unity),
            "rust" => Ok(ProjectType::Rust),
            _ => Err(format!("'{}' is not a valid project type", s)),
        }
    }
}

impl fmt::Display for ProjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProjectType::Unity => write!(f, "unity"),
            ProjectType::Rust => write!(f, "rust"),
        }
    }
}

impl ProjectType {
    pub(crate) fn from_path(project_path: &Path) -> Option<ProjectType> {
        if project_path.join("Assets").exists() && project_path.join("Packages").exists() && project_path.join("ProjectSettings").exists() {
            Some(ProjectType::Unity)
        } else if project_path.join("src").exists() && project_path.join("Cargo.toml").exists() && project_path.join("Cargo.lock").exists() {
            Some(ProjectType::Rust)
        } else {
            None
        }
    }
}