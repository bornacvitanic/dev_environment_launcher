use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Debug, Serialize, Deserialize)]
pub struct RustConfig {
    pub ide_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnityConfig {
    pub editor_base_path: PathBuf,
    pub json_editor_path: PathBuf,
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
                json_editor_path: PathBuf::new(),
            },
        };
        let toml = toml::to_string(&default_config).unwrap();
        fs::write(file, toml)
    }

    pub fn save_to_file(&self, file: &Path) -> Result<(), std::io::Error> {
        let toml = toml::to_string(self).unwrap();
        fs::write(file, toml)
    }

    pub fn get_config_dir(app_name: &str) -> PathBuf {
        match env::consts::OS {
            "windows" => PathBuf::from(env::var("APPDATA").unwrap()).join(app_name),
            "macos" => PathBuf::from(env::var("HOME").unwrap())
                .join("Library/Application Support")
                .join(app_name),
            "linux" => PathBuf::from(env::var("HOME").unwrap())
                .join(".config")
                .join(app_name),
            _ => panic!("Unsupported OS"),
        }
    }

    pub fn get_config(config_dir: &PathBuf, config_path: &PathBuf) -> Result<Self, String> {
        // Ensure the configuration directory exists
        if let Err(e) = fs::create_dir_all(config_dir) {
            return Err(format!("Failed to create config directory: {}", e));
        }

        if !Path::new(&config_path).exists() {
            if let Err(e) = Config::create_default(config_path) {
                return Err(format!(
                    "Failed to create default configuration file: {}",
                    e
                ));
            }
            println!(
                "Created default configuration file at {}",
                config_path.display()
            );
        }

        match Config::from_file(config_path) {
            Ok(config) => Ok(config),
            Err(e) => Err(format!("Failed to load configuration: {}", e)),
        }
    }
}
