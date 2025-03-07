use ggez::{
    Context,
    input::keyboard::{self, KeyCode},
};
use crate::net::NetClient;
use protocol::Position;
use crate::map::Map;

// Direction enum for player animation
#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Down,
    Left,
    Right,
    Up,
}


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
    
    if ctx.keyboard.is_key_pressed(KeyCode::Up)
        || ctx.keyboard.is_key_pressed(KeyCode::W)
    {
        dy -= MOVEMENT_SPEED;
        direction = Direction::Up;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Down)
        || ctx.keyboard.is_key_pressed(KeyCode::S)
    {
        dy += MOVEMENT_SPEED;
        direction = Direction::Down;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Left)
        || ctx.keyboard.is_key_pressed(KeyCode::A)
    {
        dx -= MOVEMENT_SPEED;
        direction = Direction::Left;
    }
    if ctx.keyboard.is_key_pressed(KeyCode::Right)
        || ctx.keyboard.is_key_pressed(KeyCode::D)
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

pub fn update_position(pos: &mut Position, movement: &MovementState, map: &Map, grid_size: f32) {
    if movement.is_moving {
        // Calculate new position
        let new_x = pos.x + movement.dx;
        let new_y = pos.y + movement.dy;
        
        // Calculate the center of the player sprite for collision detection
        // The position is the top-left corner of the sprite, so we add half the player size to get the center
        let center_x = new_x + PLAYER_SIZE / 2.0;
        let center_y = new_y + PLAYER_SIZE / 2.0;
        
        // Check horizontal movement
        if map.is_valid_position(center_x, pos.y + PLAYER_SIZE / 2.0, grid_size) {
            pos.x = new_x;
        }
        
        // Check vertical movement
        if map.is_valid_position(pos.x + PLAYER_SIZE / 2.0, center_y, grid_size) {
            pos.y = new_y;
        }

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