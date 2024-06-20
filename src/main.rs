use error::collect_error;
use internals::{
    adaptors::{
        app::base_app,
        core::{config::config, controllers::base_controller},
        framework::{
            left::{routes, server},
            right::db,
        },
    },
    ports::{
        core::{ConfigPort, EmployeeController},
        framework_left::*,
        framework_right::*,
    },
};

mod error;
mod helpers;
mod internals;
mod models;

#[tokio::main]
async fn main() -> Result<(), String> {
    // Initialize the configuration
    let config = &mut config::initialize().load_environment()?;
    // Initilize the db adaptor
    let db_adaptor = db::initialize(config.clone());
    db_adaptor
        .connect()
        .await
        .map_err(|e| collect_error(e.into()))?;
    let ctrl_adaptor = base_controller::initialize();
    let api_adaptor = base_app::initialize(config.clone(), ctrl_adaptor);
    let routes_adaptor = routes::routes::initialize(api_adaptor);
    server::server::initialize(routes_adaptor, config.clone())
        .serve()
        .await
        .map_err(|e| collect_error(e.into()))
}
