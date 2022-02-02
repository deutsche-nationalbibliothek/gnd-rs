use std::fs::read_to_string;
use std::path::PathBuf;

use serde::Deserialize;

use crate::CliResult;

#[derive(Deserialize, Default, PartialEq, Debug)]
pub struct Config {
    pub(crate) concept: ConceptConfig,
}

#[derive(Deserialize, Default, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct ConceptConfig {
    pub(crate) filter: Option<String>,
}

impl Config {
    pub fn from_file(filename: &PathBuf) -> CliResult<Config> {
        Ok(toml::from_str(&read_to_string(filename)?)?)
    }
}
