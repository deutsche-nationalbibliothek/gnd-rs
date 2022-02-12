use crate::concept::ConceptBuilder;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};

use pica::{Field, StringRecord};

pub(crate) struct PersonBuilder;

fn get_synonym(field: &Field, kind: SynKind) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind);

    if field.contains_code('a') {
        synonym.push(field.first('a'));
        synonym.push_with_prefix(field.first('d'), ", ");
        synonym.push_with_prefix(field.first('c'), " ");
    } else if field.contains_code('P') {
        synonym.push(field.first('P'));

        match (field.first('n'), field.first('l')) {
            (Some(numeration), Some(title)) => {
                synonym.push_str(&format!(" ({}, {})", numeration, title));
            }
            (Some(numeration), None) => {
                synonym.push_str(&format!(" ({})", numeration));
            }
            (None, Some(title)) => {
                synonym.push_str(&format!(" ({})", title));
            }
            (None, None) => (),
        }
    }

    synonym.build()
}

impl ConceptBuilder for PersonBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let uri = Self::uri(record, config)?;
        let mut concept = Concept::new(uri, ConceptKind::Person);

        if let Some(synonym) =
            get_synonym(record.first("028A").unwrap(), SynKind::Preferred)
        {
            concept.add_synonym(synonym);
        }

        for field in record.all("028@").unwrap_or_default() {
            if let Some(synonym) = get_synonym(field, SynKind::Alternative) {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
