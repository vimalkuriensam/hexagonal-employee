use axum::serve;
use tokio::net::TcpListener;

use crate::internals::ports::framework_left::{RoutesPort, ServerPort};

pub struct Adaptor {
    routes: Box<dyn RoutesPort>,
}

pub fn initialize(routes: Box<dyn RoutesPort>) -> Box<Adaptor> {
    Box::new(Adaptor { routes })
}

impl ServerPort for Adaptor {
    async fn serve(&self) {
        let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
        println!("server listening on port 3000");
        serve(listener, self.routes.routes().into_make_service())
            .await
            .unwrap();
    }
}
