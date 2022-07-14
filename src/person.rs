use crate::concept::ConceptBuilder;
use crate::config::TranslitChoice;
use crate::synset::SynonymBuilder;
use crate::{Concept, ConceptKind, Config, Result, SynKind, Synonym};

use lazy_static::lazy_static;
use pica::matcher::{FieldMatcher, MatcherFlags};
use pica::StringRecord;
use pica_core::Field;
use regex::Regex;

pub(crate) struct PersonBuilder;

fn get_biographical_data(record: &StringRecord) -> Option<String> {
    let matcher = FieldMatcher::new("060R.4 == 'datl'").unwrap();
    let flags = MatcherFlags::default();

    if let Some(field) =
        record.iter().find(|field| matcher.is_match(field, &flags))
    {
        match (field.first('a'), field.first('b')) {
            (Some(from), Some(to)) => Some(format!(" ({}-{})", from, to)),
            (Some(from), None) => Some(format!(" ({})", from)),
            (None, Some(to)) => Some(format!(" ({})", to)),
            _ => None,
        }
    } else {
        None
    }
}

pub(crate) fn get_synonym(
    field: &Field,
    kind: SynKind,
    translit: Option<&TranslitChoice>,
    min_length: usize,
    synonym_filter: Option<&String>,
    no_initials: bool,
) -> Option<Synonym> {
    let mut synonym = Synonym::builder(kind)
        .translit(translit)
        .min_length(min_length)
        .filter(synonym_filter);

    if field.contains_code('a') {
        if no_initials && field.contains_code('d') {
            let forename = field.first('d').unwrap();

            if forename.len() == 2 && forename[1] == b'.' {
                return None;
            }
        }

        synonym = synonym
            .push(field.first('a'))
            .push_with_prefix(field.first('d'), ", ")
            .push_with_prefix(field.first('c'), " ");
    } else if field.contains_code('P') {
        synonym = synonym.push(field.first('P'));

        match (field.first('n'), field.first('l')) {
            (Some(numeration), Some(title)) => {
                synonym =
                    synonym.push_str(&format!(" ({}, {})", numeration, title));
            }
            (Some(numeration), None) => {
                synonym = synonym.push_str(&format!(" ({})", numeration));
            }
            (None, Some(title)) => {
                synonym = synonym.push_str(&format!(" ({})", title));
            }
            (None, None) => (),
        }
    }

    synonym.build()
}

impl ConceptBuilder for PersonBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        let min_length = config.concept.min_synonym_length.unwrap_or_default();
        let no_initials = config.concept.person_no_initials.unwrap_or_default();
        let synonym_filter = config.concept.synonym_filter.as_ref();
        let translit = config.concept.translit.as_ref();

        let mut concept = Concept::new(
            Self::uri(record, config)?,
            Self::relations(record, config),
            ConceptKind::Person,
        );

        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"([^,()]+),\s([^,()]+)$").unwrap();
        }

        if let Some(synonym) = get_synonym(
            record.first("028A").unwrap(),
            SynKind::Preferred,
            translit,
            min_length,
            synonym_filter,
            no_initials,
        ) {
            if let Some(captures) = RE.captures(synonym.label()) {
                if let Some(hidden_label) = SynonymBuilder::new(SynKind::Hidden)
                    .translit(translit)
                    .push_str(format!(
                        "{} {}",
                        captures.get(2).unwrap().as_str(),
                        captures.get(1).unwrap().as_str()
                    ))
                    .build()
                {
                    concept.add_synonym(hidden_label);
                }
            }

            if let Some(biographical_data) = get_biographical_data(record) {
                if let Some(pref_label) = SynonymBuilder::from(&synonym)
                    .push_str(biographical_data)
                    .build()
                {
                    concept.add_synonym(pref_label);

                    if let Some(alt_label) = SynonymBuilder::from(&synonym)
                        .kind(SynKind::Hidden)
                        .build()
                    {
                        concept.add_synonym(alt_label);
                    }
                }
            } else {
                concept.add_synonym(synonym);
            }
        }

        for field in record.all("028@").unwrap_or_default() {
            if let Some(synonym) = get_synonym(
                field,
                SynKind::Alternative,
                translit,
                min_length,
                synonym_filter,
                no_initials,
            ) {
                if let Some(captures) = RE.captures(synonym.label()) {
                    if let Some(hidden_label) =
                        SynonymBuilder::new(SynKind::Hidden)
                            .translit(translit)
                            .push_str(format!(
                                "{} {}",
                                captures.get(2).unwrap().as_str(),
                                captures.get(1).unwrap().as_str()
                            ))
                            .build()
                    {
                        concept.add_synonym(hidden_label);
                    }
                }

                concept.add_synonym(synonym);
            }
        }

        Ok(concept)
    }
}
