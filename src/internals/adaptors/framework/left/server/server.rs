use std::collections::HashMap;

use axum::serve;
use tokio::net::TcpListener;

use crate::{helpers::load_env_var, internals::ports::framework_left::{RoutesPort, ServerPort}, ConfigPort};

use super::server_error::ServerError;

pub struct Adaptor {
    routes: Box<dyn RoutesPort>,
    config: Box<dyn ConfigPort>
}

pub fn initialize(routes: Box<dyn RoutesPort>, config: Box<dyn ConfigPort>) -> Box<Adaptor> {
    Box::new(Adaptor { routes, config })
}

fn load_key(key: &str, env: &HashMap<String, String>) -> Result<String, ServerError> {
    let var = load_env_var(&String::from(key), env)
        .map_err(|e| ServerError::KeyNotFound(e))?
        .clone();
    Ok(var)
}

impl ServerPort for Adaptor {
    async fn serve(&self) -> Result<(), ServerError> {
        let env = self.config.get_config().env;
        let port = load_key("port", &env)?;
        let address = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(address.clone()).await.unwrap();
        println!("server listening on {}", address);
        serve(listener, self.routes.routes().into_make_service())
            .await
            .unwrap();
        Ok(())
    }
}
