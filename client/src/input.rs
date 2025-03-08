use crate::net::NetClient;
use ggez::{Context, input::keyboard::KeyCode};
use protocol::Position;
use protocol::Facing::*;

// Game constants
pub const MOVEMENT_SPEED: f32 = 1.0;
pub const WORLD_SIZE: f32 = 800.0;
pub const PLAYER_SIZE: f32 = 16.0;

pub struct MovementState {
    pub is_moving: bool,
    pub direction: protocol::Facing,
    pub dx: f32,
    pub dy: f32,
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
    let mut dx = 0.0;
    let mut dy = 0.0;
    let mut direction = protocol::Facing::South;

    if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
        dy -= MOVEMENT_SPEED;
        direction = North;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S) {
        dy += MOVEMENT_SPEED;
        direction =South;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A) {
        dx -= MOVEMENT_SPEED;
        direction = West;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Right) || ctx.keyboard.is_key_pressed(KeyCode::D) {
        dx += MOVEMENT_SPEED;
        direction = East;
    }

    let is_moving = dx != 0.0 || dy != 0.0;

    MovementState {
        is_moving,
        direction,
        dx,
        dy,
    }
}

pub fn send_movement_to_server(nc: &mut NetClient, movement: &MovementState) {
    if movement.is_moving {
        // Convert to integer deltas for the server
        let dx_int = movement.dx;
        let dy_int = movement.dy;

        if dx_int != 0.0 || dy_int != 0.0 {
            let pos = Position::new(dx_int, dy_int);
            let move_event = protocol::ClientToServer::AttemptPlayerMove(pos);
            let _ = nc.send(move_event);

            let dir_event = protocol::ClientToServer::AttemptPlayerFacingChange(movement.direction);
            let _ = nc.send(dir_event);
        }
    }
}
