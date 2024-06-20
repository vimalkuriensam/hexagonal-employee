use axum::{
    routing::{get, post},
    Router,
};

use crate::internals::ports::{app::EmployeeApp, framework_left::RoutesPort};

#[derive(Debug, Clone)]
pub struct Adaptor {
    api: Box<dyn EmployeeApp>,
}

pub fn initialize(api: Box<dyn EmployeeApp>) -> Box<Adaptor> {
    Box::new(Adaptor { api })
}

impl RoutesPort for Adaptor {
    fn routes(&self) -> Router {
        Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/api/v1/employee", post({
                let api = self.api.clone();
            }))
    }
}
