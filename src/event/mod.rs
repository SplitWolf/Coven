#![allow(dead_code)]
pub mod key_event;
pub mod mouse_event;
pub mod application_event;
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
#[macro_export]
macro_rules! EVENT_STRUCT_TYPE {
    ($x:tt) => {
        fn get_event_type(&self) -> super::EventType {
            super::EventType::$x
        }
        fn get_name(&self) -> &str {
            stringify!($x)
        }
        fn is_handled(&self) -> bool {
            self.handled
        }
    
        fn set_handled(&mut self, state: bool) {
            self.handled = state;
        }
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
    #[inline]
    fn is_in_category(&self, category: EventCategory) -> bool {
        self.get_category_flags() & category as i32 != 0
    } 
    fn get_event_type(&self) -> EventType;
    fn get_category_flags(&self) -> i32;
    fn get_name(&self) -> &str;
    fn is_handled(&self) -> bool;
    fn set_handled(&mut self, state: bool);
}

impl std::fmt::Display for dyn Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}