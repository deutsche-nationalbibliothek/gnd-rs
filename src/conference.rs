use crate::concept::ConceptBuilder;
use crate::config::TranslitChoice;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};
use pica::{Field, StringRecord};

const CHECK: [char; 4] = ['n', 'd', 'c', 'g'];

pub(crate) struct ConferenceBuilder;

fn get_synonym(
    field: &Field,
    kind: SynKind,
    translit: Option<&TranslitChoice>,
) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind).translit(translit);
    let mut parens = String::new();

    for subfield in field.iter() {
        let value = String::from_utf8(subfield.value().to_vec()).unwrap();

        if !CHECK.contains(&subfield.code()) && !parens.is_empty() {
            synonym = synonym.push_str(&format!(" ({})", parens));
            parens.clear();
        }

        match subfield.code() {
            'a' => {
                synonym = synonym.push_str(&value.replace('@', ""));
            }
            'x' | 'b' => {
                synonym = synonym.push_str(&format!(" / {}", value));
            }
            'g' => {
                if parens.is_empty() {
                    synonym = synonym.push_str(&format!(" ({})", value));
                } else {
                    parens.push_str(&format!(" ({})", value));
                }
            }
            'n' | 'd' | 'c' => {
                if !parens.is_empty() {
                    parens.push_str(", ");
                }
                parens.push_str(&value);
            }
            _ => continue,
        }
    }

    if !parens.is_empty() {
        synonym = synonym.push_str(&format!(" ({})", parens));
    }

    synonym.build()
}

impl ConceptBuilder for ConferenceBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let translit = config.concept.translit.as_ref();
        let mut concept = Concept::new(
            Self::uri(record, config)?,
            Self::relations(record, config),
            ConceptKind::Conference,
        );

        if let Some(synonym) = get_synonym(
            record.first("030A").unwrap(),
            SynKind::Preferred,
            translit,
        ) {
            concept.add_synonym(synonym);
        }

        for field in record.all("030@").unwrap_or_default() {
            if let Some(synonym) =
                get_synonym(field, SynKind::Alternative, translit)
            {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
