use crate::concept::ConceptBuilder;
use crate::{Concept, ConceptKind, Config, Result};
use pica::StringRecord;

pub(crate) struct WorkBuilder;

impl ConceptBuilder for WorkBuilder {
    fn from_record(record: &StringRecord, config: &Config) -> Result<Concept> {
        Ok(Concept {
            uri: Self::uri(record, config)?,
            kind: ConceptKind::Work,
        })
    }
}
