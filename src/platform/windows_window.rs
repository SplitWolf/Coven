use crate::window::{Window, WindowProps, EventCallback};
use winit::{
    window::{ Window as WinitWindow, WindowBuilder}, 
    event_loop::{EventLoop, ControlFlow}, 
    dpi::{PhysicalSize, PhysicalPosition
    }};

pub struct WindowsWindow {
    props: WindowProps,
    pub window: WinitWindow,
    pub event_loop: EventLoop<()>
}

impl Window for WindowsWindow {
    fn on_update(&self) {
        todo!()
    }

    fn get_height(&self) {
        self.props.height;
    }

    fn get_width(&self) {
        self.props.width;
    }

    fn set_event_callback<T: crate::event::Event>(&mut self, event: EventCallback<T>) {
        todo!()
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
            event_loop
        }
    }
}