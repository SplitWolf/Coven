use std::time::Duration;

use crate::{event::{application_event::WindowCloseEvent, Event, IEventListener}, window::{Window, WindowProps}};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize}, 
    event::{Event as WinitEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop}, 
    platform::pump_events::EventLoopExtPumpEvents, 
    window::{ Window as WinitWindow, WindowBuilder}
};

pub struct WindowsWindow<'window_life, T: IEventListener> {
    props: WindowProps,
    window: WinitWindow,
    event_loop: EventLoop<()>,
    event_cb: Option<&'window_life T>
}

impl<'window_life, T: IEventListener> Window<'window_life, T> for WindowsWindow<'window_life, T> {
    fn on_update(&mut self) {
        self.event_loop.pump_events(Some(Duration::ZERO),  | event, elwt| {
                match event {
                    WinitEvent::WindowEvent {
                            event: WindowEvent::CloseRequested,
                            ..
                        } => {
                            println!("The close button was pressed; stopping");
                            elwt.exit();
                            let mut close_event = WindowCloseEvent::new();
                            //TODO: Safe unwrap when no callback is set
                            self.event_cb.unwrap().on_event(&mut close_event);
                        //    std::process::exit(0);
                        },
                        WinitEvent::AboutToWait => {
                            // Application update code.
                
                            // Queue a RedrawRequested event.
                            //
                            // You only need to call this if you've determined that you need to redraw in
                            // applications which do not always need to. Applications that redraw continuously
                            // can render here instead.
                            self.window.request_redraw();
                            // self.window.set_title("Test");
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

    fn get_height(&self) {
        self.props.height;
    }

    fn get_width(&self) {
        self.props.width;
    }

    fn set_event_callback(&mut self, event_cb: &'window_life T) {
        self.event_cb = Option::from(event_cb);
    }

    fn set_vsync(&mut self, enabled: bool) {
        todo!()
    }

    fn is_vsync(&self) -> bool {
        todo!()
    }

    fn create(props: &WindowProps) -> Self where Self: Sized {
        //TODO: Remove unwraps. Look into not cloning data where possible
        let event_loop = EventLoop::new().unwrap();
        let primary_monitor = event_loop.primary_monitor().unwrap();
        let width = primary_monitor.size().width;
        let height = primary_monitor.size().height;
        event_loop.set_control_flow(ControlFlow::Poll);
        let mut window_height_pos = height-props.height;
        if window_height_pos > 0 {
            window_height_pos -= 32
        }
        let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(props.width,props.height))
        .with_title(props.title.clone())
        .with_position(PhysicalPosition::new((width-props.width)/2, (window_height_pos)/2))
        .build(&event_loop).unwrap();
        let window_props = props.clone();
        WindowsWindow {
            props: window_props,
            window,
            event_loop,
            event_cb: None
        }
    }
}