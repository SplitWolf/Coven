#![allow(unused_imports)]
use coven::Application;
use coven::*;


struct SandboxApp {
    
}

impl SandboxApp {
    fn new() -> Self {
        SandboxApp { }
    }
}

impl Application for SandboxApp {
    fn create_app() -> Self where Self: Sized {
        let app =  SandboxApp::new();
        COVEN_CORE_ERROR!("testing");
        COVEN_CLIENT_ERROR!("test");
        COVEN_CLIENT_WARN!("test");
        app
    }
}
coven::startApp!(SandboxApp);