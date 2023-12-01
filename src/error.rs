use std::fmt;

pub enum Error {
    GenericError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenericError => write!(f, "Generic error"),
        }
    }
}
