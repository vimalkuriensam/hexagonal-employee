use crate::internals::adaptors::framework::right::db_error::DBError;

pub trait DBPort {
    async fn connect(&self) -> Result<(), DBError>;
}