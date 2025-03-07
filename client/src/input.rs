use ggez::{
    Context,
    input::keyboard::{self, KeyCode},
};
use crate::net::NetClient;
use protocol::Position;

// Direction enum for player animation
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}

// Constants moved from main.rs
pub const MOVEMENT_SPEED: f32 = 2.0;
pub const WORLD_SIZE: f32 = 800.0;
pub const PLAYER_SIZE: f32 = 32.0;

pub struct MovementState {
    pub is_moving: bool,
    pub direction: Direction,
    pub dx: f32,
    pub dy: f32,
}

pub fn handle_input(ctx: &Context) -> MovementState {
    let mut dx = 0.0;
    let mut dy = 0.0;
    let mut direction = Direction::Down;
    
    if keyboard::is_key_pressed(ctx, KeyCode::Up)
        || keyboard::is_key_pressed(ctx, KeyCode::W)
    {
        dy -= MOVEMENT_SPEED;
        direction = Direction::Up;
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Down)
        || keyboard::is_key_pressed(ctx, KeyCode::S)
    {
        dy += MOVEMENT_SPEED;
        direction = Direction::Down;
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Left)
        || keyboard::is_key_pressed(ctx, KeyCode::A)
    {
        dx -= MOVEMENT_SPEED;
        direction = Direction::Left;
    }
    if keyboard::is_key_pressed(ctx, KeyCode::Right)
        || keyboard::is_key_pressed(ctx, KeyCode::D)
    {
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

pub fn update_position(pos: &mut Position, movement: &MovementState) {
    if movement.is_moving {
        // Update local position
        pos.x += movement.dx;
        pos.y += movement.dy;

        // Ensure player stays within world bounds
        pos.x = pos.x.max(0.0).min(WORLD_SIZE - PLAYER_SIZE);
        pos.y = pos.y.max(0.0).min(WORLD_SIZE - PLAYER_SIZE);
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