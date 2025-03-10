use crate::net::NetClient;
use ggez::{Context, input::keyboard::KeyCode};

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
    let mut direction = protocol::Facing::South; // Default direction
    let mut is_moving = false;

    // Check for arrow key presses and WASD
    if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
        dy -= MOVEMENT_SPEED;
        direction = protocol::Facing::North;
        is_moving = true;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S) {
        dy += MOVEMENT_SPEED;
        direction = protocol::Facing::South;
        is_moving = true;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A) {
        dx -= MOVEMENT_SPEED;
        direction = protocol::Facing::West;
        is_moving = true;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Right) || ctx.keyboard.is_key_pressed(KeyCode::D) {
        dx += MOVEMENT_SPEED;
        direction = protocol::Facing::East;
        is_moving = true;
    }

    MovementState {
        is_moving,
        direction,
        dx,
        dy,
    }
}

pub fn send_movement_to_server(
    nc: &mut NetClient,
    movement: &MovementState,
    username: &str,
    player_pos: &protocol::Position,
) {
    // Always send the username for identification
    let username_msg = format!("username {}\r\n", username);
    let _ = nc.send_str(username_msg);

    // Send facing direction to server regardless of movement
    let facing_msg = format!("face {}\r\n", movement.direction);
    let _ = nc.send_str(facing_msg);

    // Only send movement if actually moving
    if movement.is_moving {
        // Send absolute position instead of relative movement
        let move_msg = format!("pos {} {}\r\n", player_pos.x, player_pos.y);
        let _ = nc.send_str(move_msg);
    }
}
