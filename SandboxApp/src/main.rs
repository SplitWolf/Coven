#![allow(unused_imports)]
use coven::Application;
use coven::ApplicationSpecification;
use coven::*;

fn main() {
    let mut app = Application::new(ApplicationSpecification {});
    app.push_layer();
    app.run();
}

// coven::startApp!(SandboxApp);