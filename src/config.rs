use std::path::PathBuf;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RustConfig {
    pub ide_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct UnityConfig {
    pub editor_base_path: PathBuf,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rust: RustConfig,
    pub unity: UnityConfig,
}

impl Config {
    pub fn from_file(file: &str) -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            .add_source(config::File::with_name(file))
            .build()?;
        settings.try_deserialize()
    }
}