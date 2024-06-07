use internals::{
    adaptors::framework::left::{routes, server},
    ports::framework_left::ServerPort,
};

mod internals;

#[tokio::main]
async fn main() {
    let routes_adaptor = routes::routes::initialize();
    server::server::initialize(routes_adaptor).serve().await;
}
