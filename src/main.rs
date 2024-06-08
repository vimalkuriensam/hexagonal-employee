use error::collect_error;
use internals::{
    adaptors::{
        core::config::config,
        framework::{
            left::{routes, server},
            right::db,
        },
    },
    ports::{framework_left::ServerPort, framework_right::DBPort},
};

mod error;
mod helpers;
mod internals;
mod models;

#[tokio::main]
async fn main() {
    // Initialize the configuration
    let config = config::initialize();
    // Initilize the db adaptor
    let db_adaptor = db::initialize(config);
    let connection = db_adaptor.connect().await.map_err(collect_error);
    if connection.is_err() {
        panic!("{:?}", connection.err());
    }
    let routes_adaptor = routes::routes::initialize();
    server::server::initialize(routes_adaptor).serve().await;
}
