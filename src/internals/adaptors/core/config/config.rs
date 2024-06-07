use std::{collections::HashMap, sync::{Arc, Mutex}};

use sqlx::PgPool;

pub struct Config {
    pub env: Option<HashMap<String, String>>,
    pub database: Option<Arc<PgPool>>,
}

pub enum EnvironmentType {
    Development,
    Production,
    Testing,
}

pub struct Adaptor {
    environment: Option<EnvironmentType>,
    config: Arc<Mutex<Config>>,
}

pub fn initialize() -> Box<Adaptor> {
    let config = Config {
        env: None,
        database: None,
    };
    Box::new(Adaptor {
        environment: None,
        config: Arc::new(Mutex::new(config)),
    })
}
impl Adaptor {
    pub fn load_environment(&self) -> Box<Adaptor> {
        todo!()
    }
}
