use ggez::{
    GameResult,
    graphics::{self, DrawParam},
};
use protocol::Facing;
use protocol::Position;

use crate::{
    assets::AssetManager,
    input::{MovementState, PLAYER_SIZE},
    map::Map,
};
// Animation constants
const ANIMATION_FRAME_TIME: f32 = 0.15; // Slightly slower animation for better visibility
const MAX_FRAMES: usize = 4; // Knight has 4 animation frames
const IDLE_ANIMATION_DELAY: f32 = 10.0; // Seconds before switching to idle animation

// Character types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CharacterType {
    Knight,
    Archer,
    Elf,
    Lizard,
    Wizard,
}

impl CharacterType {
    // Get the folder name for this character type
    pub fn folder_name(&self) -> &'static str {
        use CharacterType::*;
        match self {
            Knight => "Knight",
            Archer => "Archer",
            Elf => "Elf",
            Lizard => "Lizard",
            Wizard => "Wizzard", // Note the spelling in the folder structure
        }
    }

    // Get the next character type in the cycle
    pub fn next(&self) -> Self {
        use CharacterType::*;
        match self {
            Knight => Archer,
            Archer => Elf,
            Elf => Lizard,
            Lizard => Wizard,
            Wizard => Knight,
        }
    }
}

pub struct Player {
    #[allow(unused)]
    pub name: String,
    pub pos: Position,
    pub current_frame: usize,
    pub frame_timer: f32,
    pub direction: Facing,
    pub is_moving: bool,
    pub idle_timer: f32,               // Track how long the player has been idle
    pub is_in_idle_animation: bool,    // Whether the player is in the idle animation
    pub character_type: CharacterType, // The current character model
}

impl Player {
    pub fn new(name: String, pos: Position) -> Self {
        Self {
            name,
            pos,
            current_frame: 0,
            frame_timer: 0.0,
            direction: Facing::South,
            is_moving: false,
            idle_timer: 0.0,
            is_in_idle_animation: false,
            character_type: CharacterType::Knight,
        }
    }

    pub fn update(&mut self, movement: &MovementState, map: &Map, grid_size: f32, delta_time: f32) {
        // Store previous state
        let was_moving = self.is_moving;
        let previous_direction = self.direction;

        // Update movement state
        self.is_moving = movement.is_moving;
        self.direction = movement.direction;

        // Reset idle timer if player starts moving or changes direction
        if ((was_moving != self.is_moving) && self.is_moving)
            || (previous_direction != self.direction)
        {
            self.idle_timer = 0.0;
            self.is_in_idle_animation = false;
            log::trace!("Player moved or changed direction, resetting idle animation state");
        }

        // Update animation if moving
        if self.is_moving {
            // Update animation frame
            self.frame_timer += delta_time;
            if self.frame_timer >= ANIMATION_FRAME_TIME {
                self.frame_timer = 0.0;
                self.current_frame = (self.current_frame + 1) % MAX_FRAMES; // Using MAX_FRAMES constant
            }
        } else {
            // When not moving, increment idle timer
            self.idle_timer += delta_time;

            // Check if we should switch to idle animation
            if self.idle_timer >= IDLE_ANIMATION_DELAY && !self.is_in_idle_animation {
                self.is_in_idle_animation = true;
                self.current_frame = 0; // Reset frame for idle animation
                self.frame_timer = 0.0;
                log::info!("Entering idle animation");
            }

            // If in idle animation, update frames continuously to loop the animation
            if self.is_in_idle_animation {
                self.frame_timer += delta_time;
                if self.frame_timer >= ANIMATION_FRAME_TIME * 3.0 {
                    // Even slower idle/sleep animation
                    self.frame_timer = 0.0;
                    let old_frame = self.current_frame;
                    self.current_frame = (self.current_frame + 1) % MAX_FRAMES; // Loop through frames
                    log::trace!(
                        "Idle animation frame changed: {} -> {}",
                        old_frame,
                        self.current_frame
                    ); // Debug output
                }
            } else {
                // Reset to first frame when not moving and not in idle animation
                self.current_frame = 0;
                self.frame_timer = 0.0;
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
        // Get the character folder name
        let character = self.character_type.folder_name();
        let _gender = "M"; // Using male characters for now

        // Get the appropriate sprite based on direction, movement state, and character type
        let asset_name = if self.is_moving {
            // For moving animations, use the run animations with the current frame
            let frame = (self.current_frame % MAX_FRAMES) + 1; // Frames are 1-indexed in our asset names
            use protocol::Facing::*;
            match self.direction {
                North => format!("{}_run_up_{}", character, frame),
                East => format!("{}_run_right_{}", character, frame), // Use right sprites but flip them
                South => format!("{}_run_down_{}", character, frame),
                West => format!("{}_run_left_{}", character, frame),
            }
        } else if self.is_in_idle_animation {
            // For idle animation, use the idle animation frames and loop through them
            let frame = (self.current_frame % MAX_FRAMES) + 1;
            format!("{}_idle_{}", character, frame)
        } else {
            // For regular idle state, use the idle sprites
            use protocol::Facing::*;
            match self.direction {
                North => format!("{}_idle_up", character),
                East => format!("{}_idle_right", character), // Use right idle but flip it
                South => format!("{}_idle_down", character),
                West => format!("{}_idle_right", character),
            }
        };

        // Debug print the asset name when in idle animation
        if self.is_in_idle_animation {
            log::trace!("Using idle animation asset: {}", asset_name);
        }

        // Draw the appropriate sprite
        if let Some(hero_asset) = asset_manager.get_asset(&asset_name) {
            // Determine if we need to flip the sprite horizontally (for left direction)
            let flip_x = self.direction == protocol::Facing::East;

            // Draw the hero sprite at the correct position
            let mut draw_params = DrawParam::default().dest([self.pos.x, self.pos.y]).scale([
                if flip_x { -1.0 } else { 1.0 } * PLAYER_SIZE / hero_asset.img.width() as f32,
                PLAYER_SIZE / hero_asset.img.height() as f32,
            ]);

            // If flipping, adjust the destination to account for the flipped sprite
            if flip_x {
                draw_params = draw_params.dest([self.pos.x + PLAYER_SIZE, self.pos.y]);
            }

            canvas.draw(&hero_asset.img, draw_params);
        } else {
            // Fallback to the old player sprite if the new assets aren't found
            let fallback_asset = format!("{}", character);
            if let Some(player_asset) = asset_manager.get_asset(&fallback_asset) {
                let draw_params = DrawParam::default().dest([self.pos.x, self.pos.y]).scale([
                    PLAYER_SIZE / player_asset.img.width() as f32,
                    PLAYER_SIZE / player_asset.img.height() as f32,
                ]);

                canvas.draw(&player_asset.img, draw_params);
            } else if let Some(player_asset) = asset_manager.get_asset("player") {
                // Ultimate fallback to the original player asset
                let draw_params = DrawParam::default().dest([self.pos.x, self.pos.y]).scale([
                    PLAYER_SIZE / player_asset.img.width() as f32,
                    PLAYER_SIZE / player_asset.img.height() as f32,
                ]);

                canvas.draw(&player_asset.img, draw_params);
            }
        }

        Ok(())
    }

    pub fn switch_character(&mut self) {
        self.character_type = self.character_type.next();
        log::info!("Switched to character: {:?}", self.character_type);
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

    // Add a method to switch the character type for the main player
    pub fn switch_character(&mut self) {
        self.self_player.switch_character();
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
