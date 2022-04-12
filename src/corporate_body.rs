use crate::concept::ConceptBuilder;
use crate::config::TranslitChoice;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};
use pica::{Field, StringRecord};

pub(crate) struct CorporateBodyBuilder;

pub(crate) fn get_synonym(
    field: &Field,
    kind: SynKind,
    translit: Option<&TranslitChoice>,
) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind).translit(translit);

    for subfield in field.iter() {
        let value = subfield.value().to_string();

        match subfield.code() {
            'a' => {
                synonym = synonym.push_str(value.replace('@', ""));
            }
            'g' => {
                synonym = synonym.push_str(&format!(" ({})", value));
            }
            'x' | 'b' => {
                synonym = synonym.push_str(&format!(" / {}", value));
            }
            'n' => {
                synonym = synonym.push_str(&format!(", {}", value));
            }
            _ => continue,
        }
    }

    synonym.build()
}

impl ConceptBuilder for CorporateBodyBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let translit = config.concept.translit.as_ref();
        let mut concept = Concept::new(
            Self::uri(record, config)?,
            Self::relations(record, config),
            ConceptKind::CorporateBody,
        );

        if let Some(synonym) = get_synonym(
            record.first("029A").unwrap(),
            SynKind::Preferred,
            translit,
        ) {
            concept.add_synonym(synonym);
        }

        for field in record.all("029@").unwrap_or_default() {
            if let Some(synonym) =
                get_synonym(field, SynKind::Alternative, translit)
            {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
