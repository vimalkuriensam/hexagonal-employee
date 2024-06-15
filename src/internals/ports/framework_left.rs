use axum::Router;

use crate::internals::adaptors::framework::left::server::server_error::ServerError;

pub trait ServerPort {
    async fn serve(&self) -> Result<(), ServerError>;
}

pub trait RoutesPort {
    fn routes(&self) -> Router;
}
