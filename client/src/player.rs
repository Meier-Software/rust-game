use ggez::{
    graphics,
    GameResult,
};
use protocol::{Facing, Position};

use crate::{
    assets::AssetManager,
    input::{MovementState, PLAYER_SIZE, WORLD_SIZE},
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

    pub fn update(&mut self, movement: &MovementState, map: &Map, grid_size: i32, delta_time: f32) {
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
            let center_x = new_x + PLAYER_SIZE / 2;
            let center_y = new_y + PLAYER_SIZE / 2;

            // Check horizontal movement
            if map.is_valid_position(center_x, self.pos.y + PLAYER_SIZE / 2, grid_size) {
                self.pos.x = new_x;
            }

            // Check vertical movement
            if map.is_valid_position(self.pos.x + PLAYER_SIZE / 2, center_y, grid_size) {
                self.pos.y = new_y;
            }

            // Ensure player stays within world bounds
            self.pos.x = self.pos.x.clamp(0, WORLD_SIZE - PLAYER_SIZE);
            self.pos.y = self.pos.y.clamp(0, WORLD_SIZE - PLAYER_SIZE);
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

        log::info!("Trying to draw player with asset: {}", asset_name);

        // Draw the appropriate sprite
        if let Some(hero_asset) = asset_manager.get_asset(&asset_name) {
            log::info!("Found hero asset: {} with dimensions {}x{}", 
                      asset_name, hero_asset.img.width(), hero_asset.img.height());
            
            // Determine if we need to flip the sprite horizontally for left-facing sprites
            // Since we're using the same sprites for both left and right, we need to flip the left-facing ones
            let flip_x = self.direction == protocol::Facing::West;

            // Use a smaller scale factor to make the sprite half the size
            let scale_factor = 0.75;
            
            // Draw the hero sprite at the correct position
            let mut draw_params = graphics::DrawParam::default()
                .dest([self.pos.x as f32, self.pos.y as f32]);
                
            // Apply scaling
            if flip_x {
                // For flipped sprites, we need to adjust the destination point
                // First calculate the width of the scaled sprite
                let scaled_width = hero_asset.img.width() as f32 * scale_factor;
                
                // Set the destination to account for the flipped sprite
                draw_params = draw_params
                    .dest([self.pos.x as f32 + scaled_width, self.pos.y as f32])
                    .scale([scale_factor * -1.0, scale_factor]);
            } else {
                draw_params = draw_params.scale([scale_factor, scale_factor]);
            }

            canvas.draw(&hero_asset.img, draw_params);
            log::info!("Drew player sprite at ({}, {})", self.pos.x, self.pos.y);
            
            // Draw player name above the sprite
            let name_text = graphics::Text::new(&self.name);
            let name_width = name_text.dimensions(ctx).unwrap().w;
            canvas.draw(
                &name_text,
                graphics::DrawParam::default()
                    .dest([
                        self.pos.x as f32 + (hero_asset.img.width() as f32 * scale_factor / 2.0) - (name_width / 2.0),
                        self.pos.y as f32 - 20.0
                    ])
                    .color(graphics::Color::WHITE),
            );
        } else {
            // Fallback to the old player sprite if the new assets aren't found
            let fallback_asset = character.to_string();
            log::info!("Asset {} not found, trying fallback: {}", asset_name, fallback_asset);
            
            if let Some(player_asset) = asset_manager.get_asset(&fallback_asset) {
                log::info!("Found fallback asset: {} with dimensions {}x{}", 
                          fallback_asset, player_asset.img.width(), player_asset.img.height());
                
                // Use a smaller scale factor for the fallback sprite as well
                let scale_factor = 0.75;
                
                let draw_params = graphics::DrawParam::default()
                    .dest([self.pos.x as f32, self.pos.y as f32])
                    .scale([scale_factor, scale_factor]);
                    
                canvas.draw(&player_asset.img, draw_params);
                log::info!("Drew fallback player sprite at ({}, {})", self.pos.x, self.pos.y);
            } else {
                log::warn!("Could not find asset for player: {} or fallback: {}", asset_name, fallback_asset);
                
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
                        .color(color)
                );
                
                log::info!("Drew colored rectangle for player at ({}, {})", self.pos.x, self.pos.y);
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
                log::info!("Updating existing player: {} at position ({}, {})", name, pos.x, pos.y);
                player.pos = pos;
                player.direction = facing;
                player.is_moving = true; // Assume they're moving since we got an update
                return;
            }
        }
        
        // Player doesn't exist, add a new one
        log::info!("Adding new player: {} at position ({}, {})", name, pos.x, pos.y);
        let mut new_player = Player::new(name, pos);
        new_player.direction = facing;
        self.other_players.push(new_player);
        log::info!("Total players now: {} (including self)", self.other_players.len() + 1);
        
        // Debug print all players
        self.debug_print_players();
    }
    
    // Remove a player by name
    pub fn remove_player(&mut self, name: &str) {
        log::info!("Removing player: {}", name);
        let before_count = self.other_players.len();
        self.other_players.retain(|player| player.name != name);
        let after_count = self.other_players.len();
        
        if before_count != after_count {
            log::info!("Player {} removed. Total players now: {} (including self)", name, after_count + 1);
        } else {
            log::warn!("Player {} not found for removal", name);
        }
        
        // Debug print all players
        self.debug_print_players();
    }
    
    // Update a player's position and facing
    pub fn update_player_position(&mut self, name: &str, pos: Position, facing: Facing) {
        for player in &mut self.other_players {
            if player.name == name {
                log::info!("Updating player position: {} to ({}, {})", name, pos.x, pos.y);
                player.pos = pos;
                player.direction = facing;
                player.is_moving = true; // They're moving since we got an update
                // Reset the frame timer to animate movement
                player.frame_timer = 0.0;
                return;
            }
        }
        
        // If we didn't find the player, add them
        log::info!("Player {} not found for position update, adding new player", name);
        self.add_or_update_player(name.to_string(), pos, facing);
    }
    
    // Debug method to print all players
    pub fn debug_print_players(&self) {
        log::info!("--- Current Players ---");
        log::info!("Self: {} at ({}, {})", self.self_player.name, self.self_player.pos.x, self.self_player.pos.y);
        
        for (i, player) in self.other_players.iter().enumerate() {
            log::info!("Other[{}]: {} at ({}, {})", i, player.name, player.pos.x, player.pos.y);
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
        log::info!("Drawing {} other players", self.other_players.len());
        for player in &self.other_players {
            log::info!("Drawing player: {} at ({}, {})", player.name, player.pos.x, player.pos.y);
            player.draw(ctx, canvas, asset_manager)?;
        }

        Ok(())
    }
}
