use crate::{internals::adaptors::core::config::config::Adaptor, models::base_models::Config};

pub trait EmployeeController {}

pub trait CreateEmployeeController {
    fn create_employee();
}

pub trait ConfigPort {
    fn load_environment(&mut self) -> Result<Box<Adaptor>, String>;
    fn get_config(&self) -> Config;
}
