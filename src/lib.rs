mod concept;
mod config;
mod error;

pub use concept::{Concept, ConceptKind};
pub use config::Config;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
