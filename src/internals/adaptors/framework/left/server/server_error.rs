use std::fmt::Display;

#[derive(Debug)]
pub enum ServerError {
    KeyNotFound(String),
}

impl Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerError::KeyNotFound(msg) => write!(f, "server error: {}", msg),
        }
    }
}
