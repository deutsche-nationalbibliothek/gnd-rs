use std::collections::HashSet;
use std::fmt;

use bstr::BString;
use regex::Regex;
use unicode_normalization::UnicodeNormalization;

use crate::config::TranslitChoice;

#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Synonym {
    label: String,
    kind: SynKind,
}

impl Synonym {
    pub fn new<S>(label: S, kind: SynKind) -> Self
    where
        S: Into<String>,
    {
        Self {
            label: label.into(),
            kind,
        }
    }

    pub fn builder(kind: SynKind) -> SynonymBuilder {
        SynonymBuilder::new(kind)
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> &SynKind {
        &self.kind
    }
}

pub struct SynonymBuilder {
    buffer: String,
    kind: SynKind,
    translit: Option<TranslitChoice>,
    min_length: usize,
    filter: Option<Regex>,
}

impl SynonymBuilder {
    pub fn new(kind: SynKind) -> SynonymBuilder {
        let buffer = String::with_capacity(64);
        Self {
            buffer,
            kind,
            translit: None,
            min_length: 0,
            filter: None,
        }
    }

    pub fn kind(mut self, kind: SynKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn translit(mut self, translit: Option<&TranslitChoice>) -> Self {
        self.translit = translit.cloned();
        self
    }

    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }
    pub fn filter(mut self, filter: Option<&String>) -> Self {
        self.filter = filter.map(|s| Regex::new(s).unwrap());
        self
    }

    pub fn push(mut self, value: Option<&BString>) -> Self {
        if let Some(value) = value {
            self.buffer.push_str(&value.to_string());
        }

        self
    }

    pub fn push_str(mut self, value: impl AsRef<str>) -> Self {
        self.buffer.push_str(value.as_ref());
        self
    }

    pub fn push_with_prefix<S>(
        mut self,
        value: Option<&BString>,
        prefix: S,
    ) -> Self
    where
        S: AsRef<str>,
    {
        if let Some(value) = value {
            self.buffer.push_str(prefix.as_ref());
            self.buffer.push_str(&value.to_string());
        }

        self
    }

    pub fn build(self) -> Option<Synonym> {
        if !self.buffer.is_empty() {
            let label = match self.translit {
                Some(TranslitChoice::Nfc) => {
                    self.buffer.nfc().collect::<String>()
                }
                Some(TranslitChoice::Nfd) => {
                    self.buffer.nfd().collect::<String>()
                }
                Some(TranslitChoice::Nfkc) => {
                    self.buffer.nfkc().collect::<String>()
                }
                Some(TranslitChoice::Nfkd) => {
                    self.buffer.nfkd().collect::<String>()
                }
                _ => self.buffer,
            };

            if label.len() < self.min_length {
                return None;
            }

            if let Some(filter) = self.filter {
                if filter.is_match(&label) {
                    return None;
                }
            }

            Some(Synonym::new(&label, self.kind))
        } else {
            None
        }
    }
}

impl From<&Synonym> for SynonymBuilder {
    fn from(synonym: &Synonym) -> Self {
        SynonymBuilder::new(synonym.kind().to_owned()).push_str(synonym.label())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SynKind {
    Preferred,
    Alternative,
    Hidden,
}

impl fmt::Display for SynKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preferred => write!(f, "preferred"),
            Self::Alternative => write!(f, "alternative"),
            Self::Hidden => write!(f, "hidden"),
        }
    }
}

pub type SynSet = HashSet<Synonym>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synkind_to_string() {
        assert_eq!(SynKind::Preferred.to_string(), "preferred");
        assert_eq!(SynKind::Alternative.to_string(), "alternative");
        assert_eq!(SynKind::Hidden.to_string(), "hidden");
    }
}
