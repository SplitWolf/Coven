#![allow(unused_imports)]
use std::hint;
use std::thread::sleep;
use std::time::Duration;

use crate::event::Event;
use crate::event::application_event::WindowCloseEvent;
use crate::platform::windows_window;
use crate::window::{Window, WindowProps};
use crate::log::{self, COVEN_CORE_WARN};

pub fn init() {
    crate::log::init();
}

pub struct ApplicationSpecification {

}
pub struct Application {
    spec: ApplicationSpecification,
    m_Running: bool
}

impl Application {
    pub fn new(spec: ApplicationSpecification) -> Self {
        Application {
            spec,
            m_Running: true
        }
    }
    pub fn run(&mut self) {
        crate::init();

        while self.m_Running {
            sleep(Duration::from_millis(2000));
            self.m_Running = false;
        }

        COVEN_CORE_WARN!("testing 123");
    }
    pub fn push_layer(&mut self) {
    }
}

pub trait AppInstance {
    fn create_app() -> Self where Self: Sized;
}
