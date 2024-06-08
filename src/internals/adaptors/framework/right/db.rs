use std::io::Error;

use sqlx::{postgres::PgConnectOptions, Connection, PgConnection};

use crate::internals::ports::{core::ConfigPort, framework_right::DBPort};

use super::db_error::DBError;

pub struct Adaptor {
    config: Box<dyn ConfigPort>,
}

pub fn initialize(config: Box<dyn ConfigPort>) -> Box<Adaptor> {
    Box::new(Adaptor { config })
}

impl Adaptor {
    async fn connect(&self) -> Result<(), DBError> {
        let env = self.config.get_config().env;
        let port = env
            .get("port")
            .ok_or_else(|| DBError::KeyNotFound("port key not found".to_string()))?
            .clone()
            .parse::<u16>()
            .map_err(|_| DBError::InvalidFormat)?;
        // let db_url = "";
        // let connection = PgConnection::connect(&db_url).await.unwrap();
        // let conn = PgConnectOptions::new()
        //     .host("secret")
        //     .port(port)
        //     .username(username)
        //     .password(password)
        //     .ssl_mode(mode);
        todo!()
    }
}
