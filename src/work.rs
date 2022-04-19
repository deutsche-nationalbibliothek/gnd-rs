use crate::concept::ConceptBuilder;
use crate::config::TranslitChoice;
use crate::synset::SynonymBuilder;
use crate::{
    conference, corporate_body, person, place, Concept, ConceptKind, Config,
    Result, SynKind, Synonym,
};
use pica::matcher::{MatcherFlags, SubfieldMatcher};
use pica::{Field, StringRecord};

pub(crate) struct WorkBuilder;

fn get_synonym(
    field: &Field,
    kind: SynKind,
    translit: Option<&TranslitChoice>,
    min_length: usize,
) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind)
        .translit(translit)
        .min_length(min_length);

    for subfield in field.iter() {
        let value = subfield.value().to_string();

        match subfield.code() {
            'a' => {
                synonym = synonym.push_str(&value.replace('@', ""));
            }
            'g' => {
                synonym = synonym.push_str(&format!(" ({})", value));
            }
            'n' => {
                synonym = synonym.push_str(&format!(" {}", value));
            }
            'h' | 'f' => {
                synonym = synonym.push_str(&format!(", {}", value));
            }
            'p' | 's' | 'x' => {
                synonym = synonym.push_str(&format!(" / {}", value));
            }
            _ => continue,
        }
    }

    synonym.build()
}

fn get_prefix(record: &StringRecord) -> Option<String> {
    let matcher =
        SubfieldMatcher::new("4 in ['aut1', 'kom1', 'kue1']").unwrap();
    let flags = MatcherFlags::default();

    for tag in &["028R", "065R", "029R", "030R"] {
        for field in record.all(tag).unwrap_or_default() {
            if field
                .iter()
                .any(|subfield| matcher.is_match(subfield, &flags))
            {
                let result = match *tag {
                    "028R" => {
                        person::get_synonym(field, SynKind::Hidden, None, 0)
                    }
                    "029R" => corporate_body::get_synonym(
                        field,
                        SynKind::Hidden,
                        None,
                        0,
                    ),
                    "030R" => {
                        conference::get_synonym(field, SynKind::Hidden, None, 0)
                    }
                    "065R" => {
                        place::get_synonym(field, SynKind::Hidden, None, 0)
                    }
                    _ => unreachable!(),
                };

                if let Some(synonym) = result {
                    return Some(synonym.label().to_owned());
                }
            }
        }
    }

    None
}

impl ConceptBuilder for WorkBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let min_length = config.concept.min_synonym_length.unwrap_or_default();
        let translit = config.concept.translit.as_ref();

        let mut concept = Concept::new(
            Self::uri(record, config)?,
            Self::relations(record, config),
            ConceptKind::Work,
        );

        if let Some(synonym) = get_synonym(
            record.first("022A").unwrap(),
            SynKind::Preferred,
            translit,
            min_length,
        ) {
            if let Some(prefix) = get_prefix(record) {
                if let Some(synonym) = SynonymBuilder::new(SynKind::Preferred)
                    .push_str(&format!("{} : {}", prefix, synonym.label()))
                    .build()
                {
                    concept.add_synonym(synonym);
                }
            } else {
                concept.add_synonym(synonym);
            }
        }

        for field in record.all("022@").unwrap_or_default() {
            if let Some(synonym) =
                get_synonym(field, SynKind::Alternative, translit, min_length)
            {
                if let Some(prefix) = get_prefix(record) {
                    if let Some(synonym) =
                        SynonymBuilder::new(SynKind::Alternative)
                            .push_str(&format!(
                                "{} : {}",
                                prefix,
                                synonym.label()
                            ))
                            .build()
                    {
                        concept.add_synonym(synonym);
                    }
                } else {
                    concept.add_synonym(synonym);
                }
            }
        }

        Ok(concept)
    }
}
