use ggez::{
    GameResult,
    graphics::{self, DrawParam, Rect},
};
use protocol::Position;

use crate::{
    assets::AssetManager,
    input::{Direction, MovementState, PLAYER_SIZE},
    map::Map,
};

// Animation constants
const SPRITE_SHEET_WIDTH: f32 = 64.0;
const SPRITE_SHEET_HEIGHT: f32 = 64.0;
const ANIMATION_FRAME_TIME: f32 = 0.05;

pub struct Player {
    pub name: String,
    pub pos: Position,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub direction: Direction,
    pub is_moving: bool,
}

impl Player {
    pub fn new(name: String, pos: Position) -> Self {
        Self {
            name,
            pos,
            current_frame: 0,
            frame_timer: 0.0,
            direction: Direction::Down,
            is_moving: false,
        }
    }

    pub fn update(&mut self, movement: &MovementState, map: &Map, grid_size: f32, delta_time: f32) {
        // Update movement state
        self.is_moving = movement.is_moving;
        self.direction = movement.direction;

        // Update animation if moving
        if self.is_moving {
            // Update animation frame
            self.frame_timer += delta_time;
            if self.frame_timer >= ANIMATION_FRAME_TIME {
                self.frame_timer = 0.0;
                self.current_frame = (self.current_frame + 1) % 9; // Assuming 9 frames per direction
            }
        }

        // Update position
        if movement.is_moving {
            // Calculate new position
            let new_x = self.pos.x + movement.dx;
            let new_y = self.pos.y + movement.dy;

            // Calculate the center of the player sprite for collision detection
            let center_x = new_x + PLAYER_SIZE / 2.0;
            let center_y = new_y + PLAYER_SIZE / 2.0;

            // Check horizontal movement
            if map.is_valid_position(center_x, self.pos.y + PLAYER_SIZE / 2.0, grid_size) {
                self.pos.x = new_x;
            }

            // Check vertical movement
            if map.is_valid_position(self.pos.x + PLAYER_SIZE / 2.0, center_y, grid_size) {
                self.pos.y = new_y;
            }

            // Ensure player stays within world bounds
            const WORLD_SIZE: f32 = 800.0;
            self.pos.x = self.pos.x.max(0.0).min(WORLD_SIZE - PLAYER_SIZE);
            self.pos.y = self.pos.y.max(0.0).min(WORLD_SIZE - PLAYER_SIZE);
        }
    }

    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        // Get player sprite
        if let Some(player_asset) = asset_manager.get_asset("player") {
            // Calculate the source rectangle for the current animation frame
            let direction_offset = match self.direction {
                Direction::Up => 0,
                Direction::Left => 1,
                Direction::Down => 2,
                Direction::Right => 3,
            };

            // Each row in the sprite sheet represents a direction
            // Each column represents an animation frame
            let frame_to_use = if self.is_moving {
                self.current_frame
            } else {
                0
            };
            let src_x = (frame_to_use as f32) * SPRITE_SHEET_WIDTH;
            let src_y = (direction_offset as f32) * SPRITE_SHEET_HEIGHT;

            let src_rect = Rect::new(
                src_x / player_asset.img.width() as f32,
                src_y / player_asset.img.height() as f32,
                SPRITE_SHEET_WIDTH / player_asset.img.width() as f32,
                SPRITE_SHEET_HEIGHT / player_asset.img.height() as f32,
            );

            // Draw the player sprite at the correct position
            let draw_params = DrawParam::default()
                .dest([self.pos.x, self.pos.y])
                .scale([
                    PLAYER_SIZE / SPRITE_SHEET_WIDTH,
                    PLAYER_SIZE / SPRITE_SHEET_HEIGHT,
                ])
                .src(src_rect);

            canvas.draw(&player_asset.img, draw_params);
        }

        Ok(())
    }
}

pub struct Players {
    pub self_player: Player,
    pub other_players: Vec<Player>,
}

impl Players {
    pub fn new(player_name: String, start_pos: Position) -> Self {
        Self {
            self_player: Player::new(player_name, start_pos),
            other_players: Vec::new(),
        }
    }

    pub fn update(&mut self, movement: &MovementState, map: &Map, grid_size: f32, delta_time: f32) {
        self.self_player
            .update(movement, map, grid_size, delta_time);
    }

    pub fn draw(
        &self,
        canvas: &mut graphics::Canvas,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        // Draw the main player
        self.self_player.draw(canvas, asset_manager)?;

        // Draw other players
        for player in &self.other_players {
            player.draw(canvas, asset_manager)?;
        }

        Ok(())
    }
}
