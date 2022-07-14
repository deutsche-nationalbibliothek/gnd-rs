use std::fmt;
use std::str::FromStr;

use lazy_static::lazy_static;
use pica::{Path, StringRecord};

lazy_static! {
    static ref IDN_PATH: Path = Path::from_str("003@.0").unwrap();
    static ref GND_ID_PATH: Path = Path::from_str("003U.a").unwrap();
    static ref BBG_PATH: Path = Path::from_str("002@.0").unwrap();
}

use crate::conference::ConferenceBuilder;
use crate::corporate_body::CorporateBodyBuilder;
use crate::person::PersonBuilder;
use crate::place::PlaceBuilder;
use crate::subject_term::SubjectTermBuilder;
use crate::work::WorkBuilder;
use crate::{Config, Error, Relation, Result, SynSet, Synonym};

#[derive(Debug)]
pub struct Concept {
    pub(crate) uri: String,
    pub(crate) kind: ConceptKind,
    pub(crate) synset: SynSet,
    pub(crate) relations: Vec<Relation>,
}

impl Concept {
    pub fn new<S>(uri: S, relations: Vec<Relation>, kind: ConceptKind) -> Self
    where
        S: Into<String>,
    {
        Self {
            uri: uri.into(),
            synset: SynSet::new(),
            kind,
            relations,
        }
    }

    pub fn add_synonym(&mut self, synonym: Synonym) -> bool {
        self.synset.insert(synonym)
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn kind(&self) -> &ConceptKind {
        &self.kind
    }

    pub fn synset(&self) -> &SynSet {
        &self.synset
    }

    pub fn relations(&self) -> &Vec<Relation> {
        &self.relations
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConceptKind {
    Person,
    CorporateBody,
    Conference,
    Place,
    SubjectTerm,
    Work,
}

impl fmt::Display for ConceptKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConceptKind::Person => write!(f, "Person"),
            ConceptKind::CorporateBody => write!(f, "Corporate Body"),
            ConceptKind::Conference => write!(f, "Conference"),
            ConceptKind::Place => write!(f, "Place"),
            ConceptKind::SubjectTerm => write!(f, "Subject term"),
            ConceptKind::Work => write!(f, "Work"),
        }
    }
}

pub(crate) trait ConceptBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept>;

    fn uri(record: &StringRecord, config: &Config) -> Result<String> {
        let idn = record
            .path(&IDN_PATH)
            .first()
            .map(ToString::to_string)
            .ok_or_else(|| {
                Error::Concept("could not find valid idn".to_string())
            })?;

        Ok(config.concept.base_uri.to_owned() + &idn)
    }

    fn relations(record: &StringRecord, config: &Config) -> Vec<Relation> {
        let result = ["022R", "028R", "029R", "030R", "041R", "065R"]
            .iter()
            .flat_map(|f| record.all(f).unwrap_or_default())
            .filter_map(|f| Relation::try_from(f).ok())
            .map(|mut r| {
                r.uri = config.concept.base_uri.to_owned() + &r.uri;
                r
            })
            .collect::<Vec<Relation>>();

        result
    }
}

impl Concept {
    pub fn from_record(
        record: &StringRecord,
        config: &Config,
    ) -> Result<Concept> {
        let bbg = record.path(&BBG_PATH).first().unwrap().to_string();

        match &bbg[1..2] {
            "p" => PersonBuilder::from_record(record, config),
            "b" => CorporateBodyBuilder::from_record(record, config),
            "f" => ConferenceBuilder::from_record(record, config),
            "g" => PlaceBuilder::from_record(record, config),
            "s" => SubjectTermBuilder::from_record(record, config),
            "u" => WorkBuilder::from_record(record, config),
            s => Err(Error::Concept(format!("unknown concept kind '{}'", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_to_string() {
        assert_eq!(ConceptKind::Person.to_string(), "Person");
        assert_eq!(ConceptKind::CorporateBody.to_string(), "Corporate Body");
        assert_eq!(ConceptKind::Conference.to_string(), "Conference");
        assert_eq!(ConceptKind::Place.to_string(), "Place");
        assert_eq!(ConceptKind::SubjectTerm.to_string(), "Subject term");
        assert_eq!(ConceptKind::Work.to_string(), "Work");
    }
}
