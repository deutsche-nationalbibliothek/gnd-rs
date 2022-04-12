use std::convert::Infallible;
use std::str::FromStr;

use pica::Field;

#[derive(Debug)]
pub struct Relation {
    pub(crate) uri: String,
    kind: RelationKind,
}

impl Relation {
    pub fn new<S: Into<String>>(uri: S, kind: RelationKind) -> Self {
        Self {
            uri: uri.into(),
            kind,
        }
    }

    pub fn uri(&self) -> &str {
        &self.uri
    }

    pub fn kind(&self) -> &RelationKind {
        &self.kind
    }
}

impl TryFrom<&Field> for Relation {
    type Error = String;

    fn try_from(field: &Field) -> Result<Self, Self::Error> {
        if !field.contains_code('9') || !field.contains_code('4') {
            return Err("missing subfield 9 and 4".to_string());
        }

        let idn = field.first('9').unwrap().to_string();
        let kind =
            RelationKind::from_str(&field.first('4').unwrap().to_string())
                .unwrap();

        Ok(Relation::new(idn, kind))
    }
}

#[derive(Debug)]
pub enum RelationKind {
    Broader,
    Narrower,
    Related,
}

impl FromStr for RelationKind {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with("ob") {
            RelationKind::Broader
        } else {
            RelationKind::Related
        })
    }
}
