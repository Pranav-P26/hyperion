// This trait will be utilized by every other HYPERION module

pub trait Module {
    // Return the module's name
    fn name(&self) -> &'static str;

    // Called once during startup for startup
    fn init(&mut self);

    // Called after init for initial preperation
    fn start(&mut self);

    // Called to perform the module's main logic
    fn run(&mut self);

    // Called before shutdown to properly stop the module
    fn stop(&mut self);
}
