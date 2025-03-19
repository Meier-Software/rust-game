use crate::Position;

// This is a teleportation link to be used by doors. hub@x20y30
pub struct ZoneLink {
    // A slash seperated list.
    pub zone: String,
    pub pos: Position,
}

impl ZoneLink {
    fn from_str(stri: &str) -> Self {
let a =      stri.split_once("@");


        Self {
            zone: String::new(),
            pos: Position::new(10, 20),
        }
    }
}
