use crate::concept::ConceptBuilder;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};
use pica::{Field, StringRecord};

pub(crate) struct PlaceBuilder;

fn get_synonym(field: &Field, kind: SynKind) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind);

    for subfield in field.iter() {
        let value = subfield.value().to_string();

        match subfield.code() {
            'a' => {
                synonym.push_str(value.replace('@', ""));
            }
            'g' | 'z' => {
                synonym.push_str(&format!(" ({})", value));
            }
            'x' => {
                synonym.push_str(&format!(" / {}", value));
            }
            _ => continue,
        }
    }

    synonym.build()
}

impl ConceptBuilder for PlaceBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let uri = Self::uri(record, config)?;
        let mut concept = Concept::new(uri, ConceptKind::Place);

        if let Some(synonym) =
            get_synonym(record.first("065A").unwrap(), SynKind::Preferred)
        {
            concept.add_synonym(synonym);
        }

        for field in record.all("065@").unwrap_or_default() {
            if let Some(synonym) = get_synonym(field, SynKind::Alternative) {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
