use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::FromStr;

use pica::{Path, StringRecord};
use regex::Regex;
use serde::Deserialize;

use crate::concept::ConceptBuilder;
use crate::subject_term::SubjectTermBuilder;
use crate::Config;

#[derive(Debug)]
pub struct Collection {
    pub(crate) name: String,
    pub(crate) base_uri: String,
    pub(crate) filter: Option<Regex>,
    pub(crate) path: Path,
    pub(crate) items: HashMap<String, Vec<String>>,
    pub(crate) minimum: Option<usize>,
    pub(crate) maximum: Option<usize>,
}

impl Collection {
    pub fn new<S>(
        name: S,
        base_uri: S,
        filter: Option<Regex>,
        path: Path,
        minimum: Option<usize>,
        maximum: Option<usize>,
    ) -> Self
    where
        S: Into<String>,
    {
        Self {
            name: name.into(),
            base_uri: base_uri.into(),
            items: HashMap::new(),
            filter,
            path,
            minimum,
            maximum,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn base_uri(&self) -> &str {
        &self.base_uri
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn items(&self) -> &HashMap<String, Vec<String>> {
        &self.items
    }

    pub fn minimum(&self) -> &Option<usize> {
        &self.minimum
    }

    pub fn maximum(&self) -> &Option<usize> {
        &self.maximum
    }

    pub fn add_record(&mut self, record: &StringRecord, config: &Config) {
        let keys = record
            .path(&self.path)
            .iter()
            .map(ToString::to_string)
            .filter(|value| {
                if let Some(filter) = &self.filter {
                    filter.is_match(value)
                } else {
                    true
                }
            })
            .map(|f| String::from(&self.base_uri) + &f)
            .collect::<Vec<String>>();

        if !keys.is_empty() {
            if let Ok(uri) = SubjectTermBuilder::uri(record, config) {
                for key in keys {
                    let mut entry = self.items.entry(key);
                    let values = match entry {
                        Entry::Vacant(vacant) => vacant.insert(vec![]),
                        Entry::Occupied(ref mut occupied) => occupied.get_mut(),
                    };

                    values.push(uri.to_owned());
                }
            }
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[serde(deny_unknown_fields)]
pub struct CollectionSpec {
    pub name: String,
    pub base_uri: String,
    pub path: String,
    pub filter: Option<String>,
    pub minimum: Option<usize>,
    pub maximum: Option<usize>,
}

impl TryFrom<&CollectionSpec> for Collection {
    type Error = String;

    fn try_from(spec: &CollectionSpec) -> Result<Self, Self::Error> {
        let path = match Path::from_str(&spec.path) {
            Ok(path) => path,
            Err(_) => return Err("Invalid path".to_string()),
        };

        let filter = if let Some(filter) = &spec.filter {
            match Regex::new(filter) {
                Ok(re) => Some(re),
                _ => return Err("Invalid Regex".to_string()),
            }
        } else {
            None
        };

        Ok(Collection::new(
            &spec.name,
            &spec.base_uri,
            filter,
            path,
            spec.minimum,
            spec.maximum,
        ))
    }
}
