use crate::concept::ConceptBuilder;
use crate::config::TranslitChoice;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};
use pica::{Field, StringRecord};

pub(crate) struct SubjectTermBuilder;

fn get_synonym(
    field: &Field,
    kind: SynKind,
    translit: Option<&TranslitChoice>,
) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind).translit(translit);

    if field.contains_code('a') {
        synonym = synonym.push(field.first('a'));

        if let Some(gs) = field.all('g') {
            synonym = synonym.push_str(&format!(
                " ({})",
                gs.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(", "),
            ));
        }

        if let Some(xs) = field.all('x') {
            synonym = synonym.push_str(&format!(
                " / {}",
                xs.iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
                    .join(" / "),
            ));
        }
    }

    synonym.build()
}

impl ConceptBuilder for SubjectTermBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let uri = Self::uri(record, config)?;
        let mut concept = Concept::new(uri, ConceptKind::SubjectTerm);
        let translit = config.concept.translit.as_ref();

        if let Some(synonym) = get_synonym(
            record.first("041A").unwrap(),
            SynKind::Preferred,
            translit,
        ) {
            concept.add_synonym(synonym);
        }

        for field in record.all("041@").unwrap_or_default() {
            if let Some(synonym) =
                get_synonym(field, SynKind::Alternative, translit)
            {
                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
