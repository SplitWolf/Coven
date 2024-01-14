//TODO: Add custom std::fmt::Display for each event
use super::Event;

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
    super::EVENT_STRUCT_TYPE!(WindowResize);
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
    super::EVENT_STRUCT_TYPE!(WindowMoved);
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
    super::EVENT_STRUCT_TYPE!(WindowClose);
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
    super::EVENT_STRUCT_TYPE!(WindowFocus);
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
    super::EVENT_STRUCT_TYPE!(WindowLostFocus);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}

// -------------------- App Events --------------------

pub struct AppTickEvent {
    handled: bool
}

impl AppTickEvent {
    pub fn new() -> AppTickEvent {
        AppTickEvent { handled: false}
    }
}

impl Event for AppTickEvent {
    super::EVENT_STRUCT_TYPE!(AppTick);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}

pub struct AppUpdateEvent {
    handled: bool
}

impl AppUpdateEvent {
    pub fn new() -> AppUpdateEvent {
        AppUpdateEvent { handled: false}
    }
}

impl Event for AppUpdateEvent {
    super::EVENT_STRUCT_TYPE!(AppUpdate);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}

pub struct AppRenderEvent {
    handled: bool
}

impl AppRenderEvent {
    pub fn new() -> AppRenderEvent {
        AppRenderEvent { handled: false}
    }
}

impl Event for AppRenderEvent {
    super::EVENT_STRUCT_TYPE!(AppRender);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryApplication as i32
    }
}


