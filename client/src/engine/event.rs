use specs::{Component, VecStorage};

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[storage(VecStorage)]
pub enum EventType {
    NetSend,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Event {
    // pub etype: EventType,
    pub event: String,
}

impl Event {
    pub fn new(event: String) -> Self {
        Self {
            event,
        }
    }
}
