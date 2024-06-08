#[derive(Debug)]
pub enum DBError {
    KeyNotFound(String),
    InvalidFormat,
}
