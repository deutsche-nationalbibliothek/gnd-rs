use crate::concept::ConceptBuilder;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};
use pica::{Field, StringRecord};

pub(crate) struct CorporateBodyBuilder;

fn get_synonym(field: &Field, kind: SynKind) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind);

    for subfield in field.iter() {
        let value = subfield.value().to_string();

        match subfield.code() {
            'a' => {
                synonym.push_str(value.replace('@', ""));
            }
            'g' => {
                synonym.push_str(&format!(" ({})", value));
            }
            'x' | 'b' => {
                synonym.push_str(&format!(" / {}", value));
            }
            'n' => {
                synonym.push_str(&format!(", {}", value));
            }
            _ => continue,
        }
    }

    synonym.build()
}

impl ConceptBuilder for CorporateBodyBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let uri = Self::uri(record, config)?;
        let mut concept = Concept::new(uri, ConceptKind::CorporateBody);

        if let Some(synonym) =
            get_synonym(record.first("029A").unwrap(), SynKind::Preferred)
        {
            concept.add_synonym(synonym);
        }

        for field in record.all("029@").unwrap_or_default() {
            if let Some(synonym) = get_synonym(field, SynKind::Alternative) {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
