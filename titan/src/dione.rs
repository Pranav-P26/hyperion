use crate::module::Module;

pub struct Dione;

impl Dione {
    pub fn new() -> Self {
        Dione
    }
}

impl Module for Dione {
    fn name(&self) -> &'static str {
        "DIONE"
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
