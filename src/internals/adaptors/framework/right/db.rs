use std::{collections::HashMap, sync::Arc};

use sqlx::{Connection, PgConnection};

use crate::{
    helpers::load_env_var,
    internals::ports::{core::ConfigPort, framework_right::DBPort},
};

use super::db_error::DBError;

pub struct Adaptor {
    config: Box<dyn ConfigPort>,
}

pub fn initialize(config: Box<dyn ConfigPort>) -> Box<Adaptor> {
    Box::new(Adaptor { config })
}

fn load_key(key: &str, env: &HashMap<String, String>) -> Result<String, DBError> {
    let var = load_env_var(&String::from(key), env)
        .map_err(|e| DBError::KeyNotFound(e))?
        .clone();
    Ok(var)
}

impl DBPort for Adaptor {
    async fn connect(&self) -> Result<(), DBError> {
        let env = &self.config.get_config().env;
        let port = load_key("port", env)?
            .parse::<u16>()
            .map_err(|_| DBError::InvalidFormat)?;
        let host = load_key("host", env)?;
        let username = load_key("username", env)?;
        let password = load_key("password", env)?;
        let db_name = load_key("db_name", env)?;
        let db_url = format!(
            "postgresql://{}:{}?dbname={}&user={}&password={}",
            host, port, db_name, username, password
        );
        let connection = PgConnection::connect(&db_url).await.unwrap();
        let mut config = self.config.get_config();
        config.database = Some(Arc::new(connection));
        Ok(())
    }
}
