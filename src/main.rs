mod dione;
mod module;

use crate::dione::Dione;
use crate::module::Module;

fn main() {
    println!("TITAN: Beginning runtime...\n");

    // Creating modules
    let mut modules: Vec<Box<dyn Module>> = vec![Box::new(Dione::new())];

    // Lifecycle: init -> start -> run -> stop
    for module in modules.iter_mut() {
        println!("TITAN: Initializing {}", module.name());
        module.init();
    }

    for module in modules.iter_mut() {
        println!("TITAN: Starting {}", module.name());
        module.start();
    }

    for module in modules.iter_mut() {
        println!("TITAN: Running {}", module.name());
        module.run();
    }

    for module in modules.iter_mut() {
        println!("TITAN: Stopping {}", module.name());
        module.stop();
    }

    println!("\nTITAN: Runtime complete...");
}
