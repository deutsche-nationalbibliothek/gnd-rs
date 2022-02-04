use std::fmt;

#[derive(Debug)]
pub enum Error {
    Config(String),
    Concept(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Config(ref s) => f.write_str(s),
            Error::Concept(ref s) => f.write_str(s),
        }
    }
}
