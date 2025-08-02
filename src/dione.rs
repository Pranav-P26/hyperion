use crate::module::Module;
use serde::Deserialize;
use serde_json;
use std::fs;

pub struct Dione;

// Fields in this struct will change based on json format
#[derive(Debug, Deserialize)]
pub struct Task {
    pub id: String,
    pub description: String,
    pub priority: u8,
}

impl Dione {
    pub fn new() -> Self {
        Dione
    }

    pub fn load_json(path: &str) -> Vec<Task> {
        let data = fs::read_to_string(path).expect("Failed to read JSON file");
        serde_json::from_str(&data).expect("Failed to parse JSON")
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

        //Sample file path; must be changed later
        let tasks = Dione::load_json("tasks.json");
        println!("Loaded {} tasks", tasks.len());
        for task in tasks {
            println!("{:?}", task);
        }
    }

    fn stop(&mut self) {
        println!("[{}] stop", self.name());
    }
}
