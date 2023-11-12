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
    // Convert to BitField
    EventCategoryApplication = BIT!(0),
    EventCategoryInput = BIT!(1),
    EventCategoryKeyboard = BIT!(2),
    EventCategoryMouse = BIT!(3),
    EventCategoryMouseButton = BIT!(4)
}

pub trait Event {
    
}