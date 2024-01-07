//TODO: Add custom std::fmt::Display for each event
use super::Event;

pub struct KeyPressedEvent {
    key_code: i32,
    repeat_count: u32,
    handled: bool
}

impl KeyPressedEvent {
    pub fn new(key_code: i32, repeat_count: u32) -> KeyPressedEvent {
        KeyPressedEvent { key_code, repeat_count, handled: false }
    }
    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl Event for KeyPressedEvent {
    super::EVENT_STRUCT_TYPE!(KeyPressed);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryInput as i32 | super::EventCategory::EventCategoryKeyboard as i32
    }

}

pub struct KeyReleasedEvent {
    key_code: i32,
    handled: bool
}

impl KeyReleasedEvent {
    pub fn new(key_code: i32) -> KeyReleasedEvent {
        KeyReleasedEvent { key_code, handled: false }
    }
    pub fn get_key_code(&self) -> i32 {
        self.key_code
    }
}

impl Event for KeyReleasedEvent {
    super::EVENT_STRUCT_TYPE!(KeyReleased);
    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryInput as i32 | super::EventCategory::EventCategoryKeyboard as i32
    }
}