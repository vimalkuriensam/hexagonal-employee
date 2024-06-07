use axum::{routing::get, Router};

use crate::internals::ports::framework_left::RoutesPort;

pub struct Adaptor {}

pub fn initialize() -> Box<Adaptor> {
    Box::new(Adaptor {})
}

impl RoutesPort for Adaptor {
    fn routes(&self) -> Router {
        Router::new().route("/", get(|| async { "Hello, World!" }))
    }
}
