use super::Event;

struct KeyPressedEvent {
    key_code: i32,
    repeat_count: u32,
    handeld: bool
}

impl KeyPressedEvent {
    fn new(key_code: i32, repeat_count: u32) -> KeyPressedEvent {
        KeyPressedEvent { key_code, repeat_count, handeld: false }
    }
}

impl Event for KeyPressedEvent {
    fn get_event_type(&self) -> super::EventType {
        super::EventType::KeyPressed
    }

    fn get_category_flags(&self) -> i32 {
        super::EventCategory::EventCategoryInput as i32 | super::EventCategory::EventCategoryKeyboard as i32
    }

    fn get_name(&self) -> &str {
        "KeyPressed"
    }

    fn is_handled(&self) -> bool {
        self.handeld
    }

    fn set_handled(&mut self, state: bool) {
        self.handeld = state;
    }
}