use crate::module::Module;

pub struct Apollo;

impl Apollo {
    pub fn new() -> Self {
        Apollo
    }
}

impl Module for Apollo {
    fn name(&self) -> &'static str {
        "APOLLO"
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
