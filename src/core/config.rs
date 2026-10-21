use super::SaoError;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default, rename = "List")]
    pub list: BTreeMap<String, String>,
}

pub fn get(path: PathBuf) -> Result<Config, SaoError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

pub fn get_default() -> Result<Config, SaoError> {
    for path in [first_path(), second_path()] {
        if let Some(path) = path {
            match get(path) {
                Err(_) => (),
                Ok(config) => return Ok(config),
            }
        }
    }
    Err(SaoError::NoConfigFile)
}

fn first_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p| {
        p.push("sao");
        p.push("config.toml");
        p
    })
}

fn second_path() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut p| {
        p.push("sao");
        p.push("config.toml");
        p
    })
}
