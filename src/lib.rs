mod collection;
mod concept;
mod conference;
mod config;
mod corporate_body;
mod error;
mod person;
mod place;
mod relation;
mod subject_term;
mod synset;
mod work;

pub use collection::Collection;
pub use concept::{Concept, ConceptKind};
pub use config::Config;
pub use error::Error;
pub use relation::{Relation, RelationKind};
pub use synset::{SynKind, SynSet, Synonym};

pub type Result<T> = std::result::Result<T, Error>;
