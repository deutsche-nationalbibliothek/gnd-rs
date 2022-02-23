use std::fs::read_to_string;
use std::path::PathBuf;

use serde::Deserialize;

use crate::{Error, Result};

#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct Config {
    pub concept: ConceptConfig,
    pub skosify: SkosifyConfig,
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConceptConfig {
    pub filter: Option<String>,
    pub base_uri: Option<String>,
    pub gnd_id: bool,
    pub skip_invalid: bool,
    pub translit: Option<TranslitChoice>,
}

impl Default for ConceptConfig {
    fn default() -> Self {
        Self {
            filter: None,
            base_uri: None,
            gnd_id: true,
            skip_invalid: false,
            translit: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum TranslitChoice {
    Nfc,
    Nfd,
    Nfkc,
    Nfkd,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct SkosifyConfig {
    pub pretty: bool,
}

impl Config {
    pub fn from_file(filename: &PathBuf) -> Result<Config> {
        let content = match read_to_string(filename) {
            Ok(content) => content,
            _ => {
                return Err(Error::Config(
                    "unable to open config file '{}'".to_string(),
                ))
            }
        };

        let config = match toml::from_str(&content) {
            Ok(config) => config,
            Err(e) => return Err(Error::Config(e.to_string())),
        };

        Ok(config)
    }
}
