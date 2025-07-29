use crate::module::Module;

pub struct Orion;

impl Orion {
    pub fn new() -> Self {
        Orion
    }
}

impl Module for Orion {
    fn name(&self) -> &'static str {
        "ORION"
    }

    fn init(&mut self) {
        println!("[{}] init", self.name());
    }

    fn start(&mut self) {
        println!("[{}] start", self.name());
    }

    fn run(&mut self) {
        println!("[{}] run", self.name());
    }

    fn stop(&mut self) {
        println!("[{}] stop", self.name());
    }
}
