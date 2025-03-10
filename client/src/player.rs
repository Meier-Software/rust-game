use ggez::{
    Context, GameResult,
    graphics::{self, Drawable},
};
use protocol::{Facing, Position};

use crate::{
    assets::AssetManager,
    input::{MovementState, PLAYER_SIZE},
    map::Map,
};
// Animation constants
const ANIMATION_FRAME_TIME: f32 = 0.15; // Slightly slower animation for better visibility
const MAX_FRAMES: usize = 4; // Knight has 4 animation frames
const IDLE_ANIMATION_DELAY: f32 = 10.0; // Seconds before switching to idle animation
const ANIMATION_SPEED: f32 = 5.0;
const IDLE_ANIMATION_SPEED: f32 = 2.0;

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
    pub chat_message: Option<String>,  // Current chat message to display
    pub chat_timer: f32,               // How long to display the chat message
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
            chat_message: None,
            chat_timer: 0.0,
        }
    }

    pub fn update(&mut self, movement: &MovementState, map: &Map, grid_size: i32, delta_time: f32) {
        // Update direction based on movement
        if movement.is_moving {
            self.direction = movement.direction;
        }

        // Calculate new position based on movement
        let new_x = self.pos.x + movement.dx;
        let new_y = self.pos.y + movement.dy;

        // Check if the new position is valid
        if map.is_valid_position(new_x, new_y, grid_size) {
            self.pos.x = new_x;
            self.pos.y = new_y;
            self.is_moving = movement.is_moving;
        } else {
            // If we can't move in both directions, try moving in just one
            let new_x_only = self.pos.x + movement.dx;
            let new_y_only = self.pos.y + movement.dy;

            if map.is_valid_position(new_x_only, self.pos.y, grid_size) {
                self.pos.x = new_x_only;
                self.is_moving = movement.is_moving;
            }

            if map.is_valid_position(self.pos.x, new_y_only, grid_size) {
                self.pos.y = new_y_only;
                self.is_moving = movement.is_moving;
            }
        }

        // Update chat message timer
        if let Some(_) = &self.chat_message {
            self.chat_timer -= delta_time;
            if self.chat_timer <= 0.0 {
                self.chat_message = None;
            }
        }

        // Update animation frame
        if self.is_moving {
            // Reset idle timer when moving
            self.idle_timer = 0.0;
            self.is_in_idle_animation = false;

            // Update frame timer for running animation
            self.frame_timer += delta_time * ANIMATION_SPEED;
            if self.frame_timer >= 1.0 {
                self.frame_timer = 0.0;
                self.current_frame = (self.current_frame + 1) % MAX_FRAMES;
            }
        } else {
            // Update idle timer
            self.idle_timer += delta_time;

            // Start idle animation after 3 seconds of no movement
            if self.idle_timer >= 3.0 && !self.is_in_idle_animation {
                self.is_in_idle_animation = true;
                self.current_frame = 0;
                self.frame_timer = 0.0;
            }

            // Update frame timer for idle animation
            if self.is_in_idle_animation {
                self.frame_timer += delta_time * IDLE_ANIMATION_SPEED;
                if self.frame_timer >= 1.0 {
                    self.frame_timer = 0.0;
                    self.current_frame = (self.current_frame + 1) % MAX_FRAMES;
                }
            }
        }
    }

    pub fn draw(
        &self,
        ctx: &Context,
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
                East => format!("{}_run_right_{}", character, frame),
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
                East => format!("{}_idle_right", character),
                South => format!("{}_idle_down", character),
                West => format!("{}_idle_left", character),
            }
        };

        // Draw the appropriate sprite
        if let Some(hero_asset) = asset_manager.get_asset(&asset_name) {
            // Determine if we need to flip the sprite horizontally for left-facing sprites
            // Since we're using the same sprites for both left and right, we need to flip the left-facing ones
            let flip_x = self.direction == protocol::Facing::West;

            // Use a smaller scale factor to make the sprite half the size
            let scale_factor = 0.75;

            // Calculate the scaled width for potential use in flipping and positioning
            let scaled_width = hero_asset.img.width() as f32 * scale_factor;

            // Draw the hero sprite at the correct position
            let mut draw_params =
                graphics::DrawParam::default().dest([self.pos.x as f32, self.pos.y as f32]);

            // Apply scaling
            if flip_x {
                // For flipped sprites, we need to adjust the destination point
                // Set the destination to account for the flipped sprite
                draw_params = draw_params
                    .dest([self.pos.x as f32 + scaled_width, self.pos.y as f32])
                    .scale([scale_factor * -1.0, scale_factor]);
            } else {
                draw_params = draw_params.scale([scale_factor, scale_factor]);
            }

            // Special handling for Archer character - adjust position if needed
            if character == "Archer" {
                // Only adjust the Y position for Archer to center it vertically
                let y_offset = 4.0; // Adjust y position for Archer

                // Get the current x position (which already accounts for flipping if needed)
                let x_pos = if flip_x {
                    self.pos.x as f32 + scaled_width
                } else {
                    self.pos.x as f32
                };

                // Apply the y offset while preserving the x position
                draw_params = draw_params.dest([x_pos, self.pos.y as f32 - y_offset]);
            }

            canvas.draw(&hero_asset.img, draw_params);

            // Calculate the center position for text elements
            let center_x = self.pos.x as f32 + (hero_asset.img.width() as f32 * scale_factor / 2.0);

            // Draw player name above the sprite
            let name_text = graphics::Text::new(&self.name);
            let name_width = name_text.dimensions(ctx).unwrap().w;
            canvas.draw(
                &name_text,
                graphics::DrawParam::default()
                    .dest([center_x - (name_width / 2.0), self.pos.y as f32 - 20.0])
                    .color(graphics::Color::WHITE),
            );

            // Draw chat message above the player name if there is one
            if let Some(message) = &self.chat_message {
                // Create a chat bubble with the message
                let chat_text = graphics::Text::new(message);
                let chat_width = chat_text.dimensions(ctx).unwrap().w;
                let chat_height = chat_text.dimensions(ctx).unwrap().h;

                // Draw chat bubble background
                let bubble_padding = 5.0;
                let bubble_rect = graphics::Rect::new(
                    center_x - (chat_width / 2.0) - bubble_padding,
                    self.pos.y as f32 - 45.0 - chat_height,
                    chat_width + (bubble_padding * 2.0),
                    chat_height + (bubble_padding * 2.0),
                );

                let bubble = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    bubble_rect,
                    graphics::Color::new(0.0, 0.0, 0.0, 0.7), // Semi-transparent black
                )
                .unwrap();

                canvas.draw(&bubble, graphics::DrawParam::default());

                // Draw chat message text
                canvas.draw(
                    &chat_text,
                    graphics::DrawParam::default()
                        .dest([
                            center_x - (chat_width / 2.0),
                            self.pos.y as f32 - 45.0 - chat_height + bubble_padding,
                        ])
                        .color(graphics::Color::WHITE),
                );
            }
        } else {
            // Fallback to the old player sprite if the new assets aren't found
            let fallback_asset = character.to_string();

            if let Some(player_asset) = asset_manager.get_asset(&fallback_asset) {
                // Use a smaller scale factor for the fallback sprite as well
                let scale_factor = 0.75;

                let draw_params = graphics::DrawParam::default()
                    .dest([self.pos.x as f32, self.pos.y as f32])
                    .scale([scale_factor, scale_factor]);

                canvas.draw(&player_asset.img, draw_params);
            } else {
                // Draw a colored rectangle as a fallback to make the player visible
                let color = match self.character_type {
                    CharacterType::Knight => graphics::Color::RED,
                    CharacterType::Archer => graphics::Color::GREEN,
                    CharacterType::Elf => graphics::Color::BLUE,
                    CharacterType::Lizard => graphics::Color::YELLOW,
                    CharacterType::Wizard => graphics::Color::MAGENTA,
                };

                // Draw a simple rectangle using the canvas's rectangle drawing method
                // Make the rectangle half the size as well
                let rect_size = PLAYER_SIZE as f32 * 0.75;
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::default()
                        .dest([self.pos.x as f32, self.pos.y as f32])
                        .scale([rect_size, rect_size])
                        .color(color),
                );
            }
        }

        Ok(())
    }

    pub fn switch_character(&mut self) {
        self.character_type = self.character_type.next();
    }

    // Add a new method to set a chat message
    pub fn set_chat_message(&mut self, message: String) {
        self.chat_message = Some(message);
        self.chat_timer = 5.0; // Display for 5 seconds
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

    pub fn update(&mut self, movement: &MovementState, map: &Map, grid_size: i32, delta_time: f32) {
        self.self_player
            .update(movement, map, grid_size, delta_time);

        // Also update other players with the same delta time
        for player in &mut self.other_players {
            // For other players, we don't apply movement from local input
            let no_movement = MovementState {
                is_moving: player.is_moving,
                direction: player.direction,
                dx: 0,
                dy: 0,
            };
            player.update(&no_movement, map, grid_size, delta_time);
        }
    }

    // Add a method to switch the character type for the main player
    pub fn switch_character(&mut self) {
        self.self_player.switch_character();
    }

    // New methods for multiplayer support

    // Add a new player or update an existing one
    pub fn add_or_update_player(&mut self, name: String, pos: Position, facing: Facing) {
        // Check if player already exists
        for player in &mut self.other_players {
            if player.name == name {
                // Update existing player
                player.pos = pos;
                player.direction = facing;
                player.is_moving = true; // Assume they're moving since we got an update
                return;
            }
        }

        // Player doesn't exist, add a new one
        let mut new_player = Player::new(name, pos);
        new_player.direction = facing;
        self.other_players.push(new_player);
    }

    // Remove a player by name
    pub fn remove_player(&mut self, name: &str) {
        self.other_players.retain(|player| player.name != name);
    }

    // Update a player's position and facing
    pub fn update_player_position(&mut self, name: &str, pos: Position, facing: Facing) {
        for player in &mut self.other_players {
            if player.name == name {
                player.pos = pos;
                player.direction = facing;
                player.is_moving = true; // They're moving since we got an update
                // Reset the frame timer to animate movement
                player.frame_timer = 0.0;
                return;
            }
        }

        // If we didn't find the player, add them
        self.add_or_update_player(name.to_string(), pos, facing);
    }

    // Debug method to print all players
    pub fn debug_print_players(&self) {
        log::info!("--- Current Players ---");
        log::info!(
            "Self: {} at ({}, {})",
            self.self_player.name,
            self.self_player.pos.x,
            self.self_player.pos.y
        );

        for (i, player) in self.other_players.iter().enumerate() {
            log::info!(
                "Other[{}]: {} at ({}, {})",
                i,
                player.name,
                player.pos.x,
                player.pos.y
            );
        }
        log::info!("----------------------");
    }

    pub fn draw(
        &self,
        ctx: &Context,
        canvas: &mut graphics::Canvas,
        asset_manager: &AssetManager,
    ) -> GameResult<()> {
        // Draw the main player
        self.self_player.draw(ctx, canvas, asset_manager)?;

        // Draw other players
        for player in &self.other_players {
            player.draw(ctx, canvas, asset_manager)?;
        }

        Ok(())
    }

    // Add a method to set a chat message for a specific player
    pub fn set_player_chat_message(&mut self, username: &str, message: String) {
        // Check if it's the self player
        if self.self_player.name == username {
            self.self_player.set_chat_message(message);
            return;
        }

        // Otherwise, find the player in other_players
        for player in &mut self.other_players {
            if player.name == username {
                player.set_chat_message(message);
                break;
            }
        }
    }
}
