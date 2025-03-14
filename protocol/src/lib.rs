pub mod client_to_server;
pub use client_to_server::ClientToServer;

pub mod server_to_client;
pub use server_to_client::ServerToClient;

pub mod core;
pub use core::{Facing, Position};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ProtocolError {
    InvalidFacingDirection,
}

// This is a teleportation link to be used by doors. hub@x20y30
pub struct ZoneLink {
    // A slash seperated list.
    pub zone: String,
    pub pos: Position,
}
