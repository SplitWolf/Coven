//TODO: Add custom std::fmt::Display for each event
use super::Event;

// Non-button events
pub struct MouseMovedEvent {
    x: f32,
    y: f32,
    handled: bool
}

impl MouseMovedEvent {
    pub fn new(x: f32, y: f32) -> MouseMovedEvent {
        MouseMovedEvent {x,y,handled: false}
    }
    pub fn get_x(&self) -> f32 {
        self.x
    }
    pub fn get_y(&self) -> f32 {
        self.y
    }
}

impl Event for MouseMovedEvent {
    super::EVENT_STRUCT_TYPE!(MouseMoved);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryMouse as i32 | super::EventCategory::EventCategoryInput as i32
    }
}

impl std::fmt::Display for MouseMovedEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} X: {} Y: {}", self.get_name(),self.get_x(),self.get_y())
    }
}

pub struct MouseScrolledEvent {
    x_offset: f32,
    y_offset: f32,
    handled: bool
}

impl MouseScrolledEvent {
    pub fn new(x_offset: f32, y_offset: f32) -> MouseScrolledEvent {
        MouseScrolledEvent {x_offset,y_offset,handled: false}
    }
    pub fn get_x_offset(&self) -> f32 {
        self.x_offset
    }
    pub fn get_y_offset(&self) -> f32 {
        self.y_offset
    }
}

impl Event for MouseScrolledEvent {
    super::EVENT_STRUCT_TYPE!(MouseScrolled);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryMouse as i32 | super::EventCategory::EventCategoryInput as i32
    }
}
// Button Events
pub struct MouseButtonPressedEvent {
    button: i32,
    handled: bool,
}

impl MouseButtonPressedEvent {
    pub fn new(button: i32) -> MouseButtonPressedEvent {
        MouseButtonPressedEvent{button,handled: false}
    }
    pub fn get_mouse_button(&self) -> i32 {
        self.button
    }
}

impl Event for MouseButtonPressedEvent {
    super::EVENT_STRUCT_TYPE!(MouseButtonPressed);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryMouse as i32 | super::EventCategory::EventCategoryInput as i32 | super::EventCategory::EventCategoryMouseButton as i32
    }
}

pub struct MouseButtonReleasedEvent {
    button: i32,
    handled: bool,
}

impl MouseButtonReleasedEvent {
    pub fn new(button: i32) -> MouseButtonReleasedEvent {
        MouseButtonReleasedEvent{button,handled: false}
    }
    pub fn get_mouse_button(&self) -> i32 {
        self.button
    }
}

impl Event for MouseButtonReleasedEvent {
    super::EVENT_STRUCT_TYPE!(MouseButtonReleased);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryMouse as i32 | super::EventCategory::EventCategoryInput as i32 | super::EventCategory::EventCategoryMouseButton as i32
    }
}