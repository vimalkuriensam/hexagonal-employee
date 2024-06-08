use crate::internals::adaptors::framework::right::db_error::DBError;

pub fn collect_error(e: DBError) -> Result<(), String> {
    match e {
        DBError::KeyNotFound(e) => Err(e),
        DBError::InvalidFormat => Err(String::from("invalid key value format"))
    }
}
