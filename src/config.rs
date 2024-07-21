use std::fs;
use std::path::{Path, PathBuf};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RustConfig {
    pub ide_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnityConfig {
    pub editor_base_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub rust: RustConfig,
    pub unity: UnityConfig,
}

impl Config {
    pub fn from_file(file: &Path) -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(file.to_str().unwrap()))
            .build()?;
        settings.try_deserialize()
    }

    pub fn create_default(file: &Path) -> Result<(), std::io::Error> {
        let default_config = Config {
            rust: RustConfig {
                ide_path: PathBuf::new(),
            },
            unity: UnityConfig {
                editor_base_path: PathBuf::new(),
            },
        };
        let toml = toml::to_string(&default_config).unwrap();
        fs::write(file, toml)
    }

    pub fn save_to_file(&self, file: &PathBuf) -> Result<(), std::io::Error> {
        let toml = toml::to_string(self).unwrap();
        fs::write(file, toml)
    }
}