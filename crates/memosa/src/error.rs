#[derive(Debug)]
pub enum Error {
    InvalidId {
        invalid_id: String,
        reason: &'static str,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidId { invalid_id, reason } => {
                write!(f, "'{}' is not a valid id: {}", invalid_id, reason)
            }
        }
    }
}

impl std::error::Error for Error {}
