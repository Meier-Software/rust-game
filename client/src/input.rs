use crate::net::NetClient;
use ggez::{Context, input::keyboard::{KeyCode, KeyInput}};

// Direction enum for player animation
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

// Game constants
pub const MOVEMENT_SPEED: f32 = 1.0;
pub const WORLD_SIZE: f32 = 800.0;
pub const PLAYER_SIZE: f32 = 16.0;

pub struct MovementState {
    pub is_moving: bool,
    pub direction: Direction,
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
    let mut direction = Direction::Down;

    if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
        dy -= MOVEMENT_SPEED;
        direction = Direction::Up;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S) {
        dy += MOVEMENT_SPEED;
        direction = Direction::Down;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A) {
        dx -= MOVEMENT_SPEED;
        direction = Direction::Left;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Right) || ctx.keyboard.is_key_pressed(KeyCode::D) {
        dx += MOVEMENT_SPEED;
        direction = Direction::Right;
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
        let dx_int = movement.dx as i32;
        let dy_int = movement.dy as i32;

        if dx_int != 0 || dy_int != 0 {
            let move_cmd = format!("move {} {}\r\n", dx_int, dy_int);
            nc.send(move_cmd);
        }
    }
}
