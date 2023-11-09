use coven_engine::coven::*;

struct SandboxApp {
    
}

impl SandboxApp {
    fn new() -> Self {
        SandboxApp { }
    }
}

impl Application for SandboxApp {
    fn create_app() -> Self where Self: Sized {
        SandboxApp::new()
    }
}

coven_engine::startApp!(SandboxApp);