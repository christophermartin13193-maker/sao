use super::SaoError;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default, rename = "List")]
    pub list: BTreeMap<String, String>,
}
// Attempts to load and deserialize the TOML configuration file from a specific path
pub fn get(path: PathBuf) -> Result<Config, SaoError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

// Searches for a valid configuration file in successive default locations
pub fn get_default() -> Result<Config, SaoError> {
    for path in [first_path(), second_path()] {
        if let Some(path) = path && let Ok(config) = get(path){
                return Ok(config)
        }
    }
    Err(SaoError::NoConfigFile)
}

// Returns the standard configuration file path in the user configuration directory ($XDG_CONFIG_HOME/sao/config.toml)
fn first_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p| {
        p.push("sao");
        p.push("config.toml");
        p
    })
}

// Returns an alternative path in the user local data directory ($XDG_DATA_HOME/sao/config.toml)
fn second_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut p| {
        p.push("sao");
        p.push("config.toml");
        p
    })
}
