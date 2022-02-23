mod concept;
mod conference;
mod config;
mod corporate_body;
mod error;
mod person;
mod place;
mod subject_term;
mod synset;
mod work;

pub use concept::{Concept, ConceptKind};
pub use config::Config;
pub use error::Error;
pub use synset::{SynKind, SynSet, Synonym};

pub type Result<T> = std::result::Result<T, Error>;
