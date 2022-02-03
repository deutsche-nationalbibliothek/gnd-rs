use std::fmt;
use std::str::FromStr;

#[derive(Debug)]
pub struct Concept {
    kind: ConceptKind,
}

impl Concept {
    pub fn kind(&self) -> &ConceptKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq)]
pub enum ConceptKind {
    Person,
    CorporateBody,
    Conference,
    Place,
    SubjectTerm,
    Work,
}

impl fmt::Display for ConceptKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConceptKind::Person => write!(f, "Person"),
            ConceptKind::CorporateBody => write!(f, "Corporate Body"),
            ConceptKind::Conference => write!(f, "Conference"),
            ConceptKind::Place => write!(f, "Place"),
            ConceptKind::SubjectTerm => write!(f, "Subject term"),
            ConceptKind::Work => write!(f, "Work"),
        }
    }
}

impl FromStr for ConceptKind {
    type Err = &'static str;

    /// Parse a concept kind from from a string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use gnd::ConceptKind;
    /// use std::str::FromStr;
    ///
    /// # fn main() { example().unwrap(); }
    /// fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kind = ConceptKind::from_str("p")?;
    ///     assert_eq!(kind, ConceptKind::Person);
    ///
    ///     Ok(())
    /// }
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "p" => Ok(ConceptKind::Person),
            "b" => Ok(ConceptKind::CorporateBody),
            "f" => Ok(ConceptKind::Conference),
            "g" => Ok(ConceptKind::Place),
            "s" => Ok(ConceptKind::SubjectTerm),
            "u" => Ok(ConceptKind::Work),
            _ => Err("invalid kind"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kind_from_str() {
        assert_eq!(ConceptKind::from_str("p").unwrap(), ConceptKind::Person);
        assert_eq!(
            ConceptKind::from_str("b").unwrap(),
            ConceptKind::CorporateBody
        );
        assert_eq!(
            ConceptKind::from_str("f").unwrap(),
            ConceptKind::Conference
        );
        assert_eq!(ConceptKind::from_str("g").unwrap(), ConceptKind::Place);
        assert_eq!(
            ConceptKind::from_str("s").unwrap(),
            ConceptKind::SubjectTerm
        );
        assert_eq!(ConceptKind::from_str("u").unwrap(), ConceptKind::Work);

        assert!(ConceptKind::from_str("x").is_err());
    }

    #[test]
    fn test_kind_to_string() {
        assert_eq!(ConceptKind::Person.to_string(), "Person");
        assert_eq!(ConceptKind::CorporateBody.to_string(), "Corporate Body");
        assert_eq!(ConceptKind::Conference.to_string(), "Conference");
        assert_eq!(ConceptKind::Place.to_string(), "Place");
        assert_eq!(ConceptKind::SubjectTerm.to_string(), "Subject term");
        assert_eq!(ConceptKind::Work.to_string(), "Work");
    }
}
