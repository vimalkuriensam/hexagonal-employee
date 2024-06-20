use crate::{
    internals::ports::{app::EmployeeApp, core::ConfigPort},
    EmployeeController,
};

#[derive(Debug, Clone)]
pub struct Adaptor {
    config: Box<dyn ConfigPort>,
    ctrl: Box<dyn EmployeeController>,
}

pub fn initialize(config: Box<dyn ConfigPort>, ctrl: Box<dyn EmployeeController>) -> Box<Adaptor> {
    Box::new(Adaptor { config, ctrl })
}

impl EmployeeApp for Adaptor {}
