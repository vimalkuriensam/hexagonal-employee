use std::fmt::Debug;

use crate::{internals::adaptors::core::config::config::Adaptor, models::base_models::Config};

pub trait EmployeeController: CreateEmployeeController + Debug {}

pub trait CreateEmployeeController {
    fn create_employee(&self);
}

pub trait ConfigPort: Debug + Send + Sync {
    fn load_environment(&mut self) -> Result<Box<Adaptor>, String>;
    fn get_config(&self) -> Config;
}
