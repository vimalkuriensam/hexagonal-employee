use crate::internals::ports::core::EmployeeController;

#[derive(Debug, Clone)]
pub struct Adaptor;

pub fn initialize() -> Box<Adaptor> {
    Box::new(Adaptor {})
}

impl EmployeeController for Adaptor {}
