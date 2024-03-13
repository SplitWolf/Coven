#![allow(unused_imports)]
use std::borrow::Borrow;
use std::cell::RefCell;
use std::hint;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;

use tracing::event;

use crate::event::key_event::KeyPressedEvent;
use crate::event::{Event, EventDispatcher, IEventHandler, IEventListener};
use crate::event::application_event::WindowCloseEvent;
use crate::platform::windows_window;
use crate::window::{Window, WindowProps};
use crate::log::{self, COVEN_CORE_INFO, COVEN_CORE_WARN};

pub fn init() {
    crate::log::init();
}

pub struct ApplicationSpecification {

}

pub struct Application {
    spec: ApplicationSpecification,
    m_Running: Mutex<bool>
}

impl Application {
    pub fn new(spec: ApplicationSpecification) -> Self {
        Application {
            spec,
            m_Running: Mutex::from(true)
        }
    }
    pub fn run(&mut self) {
        crate::init();

        let mut win  = windows_window::WindowsWindow::create(
            &WindowProps::new_with_values("Coven Engine".to_string(), 1280, 720)
        );
        
        win.set_event_callback(self);

        while *self.m_Running.lock().unwrap() {
            // COVEN_CORE_WARN!("testing 123");
            win.on_update();
            sleep(Duration::from_millis(16));
        }

    }

    pub fn push_layer(&mut self) {
    }
}

impl IEventListener for Application {
    fn on_event(&self, event: &mut dyn Event) {
        let mut ed = EventDispatcher::new(event);
        let q  = ed.dispatch::<WindowCloseEvent>(self);
        COVEN_CORE_INFO!("Close Event: {}",q)
    }
}

impl IEventHandler<WindowCloseEvent> for Application {
    fn handle(&self, event: &WindowCloseEvent) -> bool {
        *self.m_Running.lock().unwrap() = false;
        true
    }
}
// Generic Version, maybe useful in future

// impl<T: Event> IEventHandler<T> for Application {
//     fn handle(&self, event: &T) -> bool {
//         match event.get_event_type() {
//             EventType::WindowClose => {
//                 *self.m_Running.lock().unwrap() = false;
//                 true
//             }
//             EventType::KeyPressed => {
//                 todo!()
//             }
//             // Add other event types as needed
//             _ => false,
//         }
//     }
// }