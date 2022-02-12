use std::collections::HashSet;
use std::fmt;

use bstr::BString;

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
}

impl SynonymBuilder {
    pub fn new(kind: SynKind) -> SynonymBuilder {
        let buffer = String::with_capacity(64);
        Self { buffer, kind }
    }

    pub fn push(&mut self, value: Option<&BString>) -> &mut Self {
        if let Some(value) = value {
            self.buffer.push_str(&value.to_string());
        }

        self
    }

    pub fn push_str(&mut self, value: impl AsRef<str>) -> &mut Self {
        self.buffer.push_str(value.as_ref());
        self
    }

    pub fn push_with_prefix<S>(
        &mut self,
        value: Option<&BString>,
        prefix: S,
    ) -> &mut Self
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
            Some(Synonym::new(&self.buffer, self.kind))
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
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
