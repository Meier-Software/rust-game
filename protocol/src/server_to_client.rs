use crate::{Facing, Position};

#[derive(Debug)]
pub enum ServerToClient {
    EntityMoved(Position),
    // TODO: Remove pos and facing from here and move it to the above
    PlayerJoined(String, Position, Facing),
    PlayerLeft(String),
    PlayerMoved(String, Position, Facing),
    ChatMessage(String, String),
}
