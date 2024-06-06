use axum::{routing::get, serve, Router};
use tokio::net::TcpListener;

use crate::internals::ports::framework_left::RoutesPort;

pub struct Adaptor {
    routes: Box<dyn RoutesPort>,
}

pub fn initialize(routes: Box<dyn RoutesPort>) -> Box<Adaptor> {
    Box::new(Adaptor { routes })
}

impl Adaptor {
    pub async fn serve() {
        let app = Router::new().route("/", get(|| async { "Hello, World!" }));
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        serve(listener, app).await.unwrap();
    }
}
