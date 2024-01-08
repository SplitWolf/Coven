#![allow(dead_code)]
use crate::event::Event;

pub type EventCallback<E> = fn(&E) -> bool;

#[derive(Clone)]
pub struct WindowProps {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl WindowProps {
    pub fn new() -> WindowProps {
        WindowProps {
            title: "Coven Engine".to_string(),
            width: 1280,
            height: 720
        }
    }

    pub fn new_with_values(title: String, width: u32, height: u32) -> WindowProps {
        WindowProps { title, width, height }
    }
}

pub trait Window {
    fn on_update(&self);
    fn get_height(&self);
    fn get_width(&self);

    fn set_event_callback<T: Event>(&mut self, event: EventCallback<T>);
    fn set_vsync(&mut self, enabled: bool);
    fn is_vsync(&self) -> bool;

    fn create(props: &WindowProps) -> Self where Self: Sized;
}