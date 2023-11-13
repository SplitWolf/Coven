use std::fmt::write;

pub enum EventType {
    WindowClose, WindowResize, WindowFocus, WindowLostFocus, WindowMoved,
    AppTick, AppUpdate, AppRender,
    KeyPressed, KeyReleased,
    MouseButtonPressed, MouseButtonReleased, MouseMoved, MouseScrolled
}

macro_rules! BIT {
    ($x:tt) => {
        1 << $x
    };
}

pub enum EventCategory {
    EventCategoryApplication = BIT!(0),
    EventCategoryInput = BIT!(1),
    EventCategoryKeyboard = BIT!(2),
    EventCategoryMouse = BIT!(3),
    EventCategoryMouseButton = BIT!(4)
}

pub trait Event {
    fn GetEventType(&self) -> Option<EventType> {
        None
    }
    fn GetCategoryFlags(&self) -> i32 {
        0
    } 
    #[inline]
    fn IsInCategory(&self, category: EventCategory) -> bool {
        self.GetCategoryFlags() & category as i32 != 0
    } 
    fn GetName(&self) -> Option<&str> {
        None
    }
    fn IsHandled(&self) -> bool;
}

impl std::fmt::Display for dyn Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.GetName().unwrap_or("No type specified"))
    }
}