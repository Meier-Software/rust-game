use crate::net::NetClient;
use ggez::{Context, input::keyboard::KeyCode};
use protocol::Facing::*;
use protocol::Position;
use protocol::ClientToServer;

// Game constants
pub const MOVEMENT_SPEED: i32 = 1;
#[allow(unused)]
pub const WORLD_SIZE: i32 = 800;
pub const PLAYER_SIZE: i32 = 16;

pub struct MovementState {
    pub is_moving: bool,
    pub direction: protocol::Facing,
    pub dx: i32,
    pub dy: i32,
}

// Add a struct to track key press events
pub struct KeyPressState {
    pub switch_character: bool,
}

// Function to check for key press events (like 'p' for character switching)
pub fn handle_key_press(ctx: &Context) -> KeyPressState {
    let mut state = KeyPressState {
        switch_character: false,
    };

    // Check if 'p' was just pressed this frame
    if ctx.keyboard.is_key_just_pressed(KeyCode::P) {
        state.switch_character = true;
    }

    state
}

pub fn handle_input(ctx: &Context) -> MovementState {
    let mut dx = 0;
    let mut dy = 0;
    let mut direction = protocol::Facing::South;

    if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
        dy -= MOVEMENT_SPEED;
        direction = North;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S) {
        dy += MOVEMENT_SPEED;
        direction = South;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A) {
        dx -= MOVEMENT_SPEED;
        direction = West;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Right) || ctx.keyboard.is_key_pressed(KeyCode::D) {
        dx += MOVEMENT_SPEED;
        direction = East;
    }

    let is_moving = dx != 0 || dy != 0;

    MovementState {
        is_moving,
        direction,
        dx,
        dy,
    }
}

pub fn send_movement_to_server(nc: &mut NetClient, movement: &MovementState, username: &str) {
    if movement.is_moving {
        // Send movement to server - use absolute position instead of deltas
        // This ensures the server always has the correct position
        let pos = Position::new(movement.dx, movement.dy);
        let event = ClientToServer::AttemptPlayerMove(pos);
        let _ = nc.send(event);
        
        // Also send the username for identification
        let username_msg = format!("username {}", username);
        let _ = nc.send_str(username_msg);
        
        // Send facing direction to server
        let event = ClientToServer::AttemptPlayerFacingChange(movement.direction);
        let _ = nc.send(event);
    }
}
