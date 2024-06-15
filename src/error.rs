use crate::internals::adaptors::framework::{
    left::server::server_error::ServerError, right::db_error::DBError,
};

#[derive(Debug)]
pub enum CommonError {
    DBError(DBError),
    ServerError(ServerError),
}

impl From<ServerError> for CommonError {
    fn from(value: ServerError) -> Self {
        CommonError::ServerError(value)
    }
}

impl From<DBError> for CommonError {
    fn from(value: DBError) -> Self {
        CommonError::DBError(value)
    }
}

pub fn collect_error(e: CommonError) -> String {
    match e {
        CommonError::ServerError(e) => e.to_string(),
        CommonError::DBError(e) => e.to_string(),
    }
}
