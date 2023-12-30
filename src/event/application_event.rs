//TODO: Add custom std::fmt::Display for each event
use super::Event;
use crate::EVENT_CLASS_TYPE;

// --------------- Window Events -------------------

// Data Window Events
pub struct WindowResizeEvent {
    height: u32,
    width: u32,
    handled: bool
}

impl WindowResizeEvent {
    pub fn new(height: u32, width: u32) -> WindowResizeEvent {
        WindowResizeEvent { height, width, handled: false}
    }
}

impl Event for WindowResizeEvent {
    EVENT_CLASS_TYPE!(WindowResize);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}

pub struct WindowMovedEvent {
    x: f32,
    y: f32,
    handled: bool
}

impl WindowMovedEvent {
    pub fn new(x: f32, y: f32) -> WindowMovedEvent {
        WindowMovedEvent { x, y, handled: false}
    }
}

impl Event for WindowMovedEvent {
    EVENT_CLASS_TYPE!(WindowMoved);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}

// No data Window Events
pub struct WindowCloseEvent {
    handled: bool
}

impl WindowCloseEvent {
    pub fn new() -> WindowCloseEvent {
        WindowCloseEvent { handled: false}
    }
}

impl Event for WindowCloseEvent {
    EVENT_CLASS_TYPE!(WindowClose);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}
// Focus Events
pub struct WindowFocusEvent {
    handled: bool
}

impl WindowFocusEvent {
    pub fn new() -> WindowFocusEvent {
        WindowFocusEvent { handled: false}
    }
}

impl Event for WindowFocusEvent {
    EVENT_CLASS_TYPE!(WindowFocus);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}
pub struct WindowLostFocusEvent {
    handled: bool
}

impl WindowLostFocusEvent {
    pub fn new() -> WindowLostFocusEvent {
        WindowLostFocusEvent { handled: false}
    }
}

impl Event for WindowLostFocusEvent {
    EVENT_CLASS_TYPE!(WindowLostFocus);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}

