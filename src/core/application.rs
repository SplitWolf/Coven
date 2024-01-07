#![allow(unused_imports)]
use std::hint;

use crate::event::Event;
use crate::event::application_event::WindowCloseEvent;
use crate::platform::windows_window;
use crate::window::{Window, WindowProps};
use crate::log;
use crate::event;
use crate::event::mouse_event::MouseMovedEvent;
use winit::{
    event::{Event as WinitEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    monitor::MonitorHandle
};
use winit::dpi::{PhysicalPosition, PhysicalSize};

pub fn init() {
    crate::log::init();

}
pub trait Application {
    fn run(&self) {
        log::COVEN_CORE_INFO!("Engine Started");
        // let mut q = event::application_event::WindowCloseEvent::new();
        // let mut q = event::mouse_event::MouseMovedEvent::new(2.0,3.0);
        // let mut p = event::EventDispatcher::new(&mut q);
        // COVEN_CORE_INFO!("dispatch: {}",p.dispatch(func));
        //windows_window::create();
        let win = windows_window::WindowsWindow::create(
            &WindowProps::new_with_values("Coven Engine".to_string(), 1600, 900)
        );
        
        let _ = win.event_loop.run(move |event, elwt| {
            match event {
                WinitEvent::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("The close button was pressed; stopping");
                    elwt.exit();
                   // std::process::exit(0);
                },
                WinitEvent::AboutToWait => {
                    // Application update code.
        
                    // Queue a RedrawRequested event.
                    //
                    // You only need to call this if you've determined that you need to redraw in
                    // applications which do not always need to. Applications that redraw continuously
                    // can render here instead.
                    // window.request_redraw();
                    // window.set_title("Test");
                },
                WinitEvent::WindowEvent {
                    event: WindowEvent::RedrawRequested,
                    ..
                } => {
                    // It's preferable for applications that do not render continuously to render in
                    // this event rather than in AboutToWait, since rendering in here allows
                    // the program to gracefully handle redraws requested by the OS.
                },
                _ => ()
            }
        });
    }
    fn create_app() -> Self where Self: Sized;
}
