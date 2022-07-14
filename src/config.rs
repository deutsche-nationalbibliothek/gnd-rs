use std::fs::read_to_string;
use std::path::PathBuf;

use serde::Deserialize;

use crate::collection::CollectionSpec;
use crate::{Error, Result};

#[derive(Deserialize, Default, PartialEq, Eq, Debug)]
pub struct Config {
    pub concept: ConceptConfig,
    #[serde(rename = "collection", default = "Vec::new")]
    pub collections: Vec<CollectionSpec>,
    pub skosify: SkosifyConfig,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConceptConfig {
    pub filter: Option<String>,
    pub base_uri: String,
    pub skip_invalid: bool,
    pub translit: Option<TranslitChoice>,
    pub min_synonym_length: Option<usize>,
    pub synonym_filter: Option<String>,
    pub person_no_initials: Option<bool>,
    pub person_no_modern_names: Option<bool>,
}

impl Default for ConceptConfig {
    fn default() -> Self {
        Self {
            filter: None,
            base_uri: "http://d-nb.info/gnd/".to_string(),
            skip_invalid: false,
            translit: None,
            min_synonym_length: None,
            synonym_filter: None,
            person_no_initials: None,
            person_no_modern_names: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Eq, Debug, Clone)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "lowercase")]
pub enum TranslitChoice {
    Nfc,
    Nfd,
    Nfkc,
    Nfkd,
}

#[derive(Deserialize, PartialEq, Eq, Debug)]
#[serde(deny_unknown_fields)]
pub struct SkosifyConfig {
    pub pretty: bool,
    #[serde(default = "default_language_tag")]
    pub language_tag: String,
}

fn default_language_tag() -> String {
    "de".to_string()
}

impl Default for SkosifyConfig {
    fn default() -> Self {
        Self {
            pretty: true,
            language_tag: default_language_tag(),
        }
    }
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
