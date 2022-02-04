use crate::concept::ConceptBuilder;
use crate::{Concept, ConceptKind, Config, Result};
use pica::StringRecord;

pub(crate) struct SubjectTermBuilder;

impl ConceptBuilder for SubjectTermBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        Ok(Concept {
            uri: Self::uri(record, config)?,
            kind: ConceptKind::SubjectTerm,
        })
    }
}