pub struct Adaptor {}

pub fn initialize() -> Box<Adaptor> {
    Box::new(Adaptor {})
}

impl Adaptor {
    fn connect(&self) {}
}
