use crate::concept::ConceptBuilder;
use crate::config::TranslitChoice;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};
use pica::StringRecord;
use pica_core::Field;

pub(crate) struct SubjectTermBuilder;

fn get_synonym(
    field: &Field,
    kind: SynKind,
    translit: Option<&TranslitChoice>,
    min_length: usize,
    synonym_filter: Option<&String>,
) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind)
        .translit(translit)
        .min_length(min_length)
        .filter(synonym_filter);

    if field.contains_code('a') {
        for subfield in field.iter() {
            let value = subfield.value().to_string();

            match subfield.code() {
                'a' => {
                    synonym = synonym.push_str(value);
                }
                'g' => {
                    synonym = synonym.push_str(&format!(" ({})", value));
                }
                'x' => {
                    synonym = synonym.push_str(&format!(" / {}", value));
                }
                _ => continue,
            }
        }
    }

    synonym.build()
}

impl ConceptBuilder for SubjectTermBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let uri = Self::uri(record, config)?;
        let relations = Self::relations(record, config);
        let mut concept =
            Concept::new(uri, relations, ConceptKind::SubjectTerm);
        let min_length = config.concept.min_synonym_length.unwrap_or_default();
        let synonym_filter = config.concept.synonym_filter.as_ref();
        let translit = config.concept.translit.as_ref();

        if let Some(synonym) = get_synonym(
            record.first("041A").unwrap(),
            SynKind::Preferred,
            translit,
            min_length,
            synonym_filter,
        ) {
            concept.add_synonym(synonym);
        }

        for field in record.all("041@").unwrap_or_default() {
            if let Some(synonym) = get_synonym(
                field,
                SynKind::Alternative,
                translit,
                min_length,
                synonym_filter,
            ) {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
