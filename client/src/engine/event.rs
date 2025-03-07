use specs::{Component, VecStorage};

#[derive(Component, Clone, Copy, Debug)]
#[storage(VecStorage)]
pub enum EventType {
    NetSend,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Event {
    pub etype: EventType,
    pub event: String,
}

impl Event {
    pub fn new(event_type: EventType, event: String) -> Self {
        Self {
            etype: event_type,
            event,
        }
    }
}
