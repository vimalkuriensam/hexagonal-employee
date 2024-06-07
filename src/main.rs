use internals::{
    adaptors::{core::config::config, framework::left::{routes, server}},
    ports::framework_left::ServerPort,
};

mod internals;
mod models;

#[tokio::main]
async fn main() {
    let config = config::initialize();
    let routes_adaptor = routes::routes::initialize();
    server::server::initialize(routes_adaptor).serve().await;
}
