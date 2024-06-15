use std::fmt::Display;

#[derive(Debug)]
pub enum DBError {
    KeyNotFound(String),
    InvalidFormat,
    ConnectionRefused(String),
}

impl Display for DBError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DBError::KeyNotFound(msg) => write!(f, "db error: {}", msg),
            DBError::InvalidFormat => write!(f, "db error: invalid format"),
            DBError::ConnectionRefused(msg) => write!(f, "db error: connection refused: {}", msg),
        }
    }
}
