use std::str::FromStr;

use ggez::{
    GameResult,
    context::Context,
    graphics::{self, Color, DrawParam, Drawable, Rect, Text},
    input::keyboard::KeyCode,
};
use protocol::Position;

use crate::{
    assets::AssetManager,
    input::{self, MovementState},
    map::Map,
    net::NetClient,
    player::Players,
};

// Constants
pub const GRID_SIZE: i32 = 16;
pub const CAMERA_ZOOM: f32 = 3.5;
#[allow(unused)]
pub const DIALOGUE_PADDING: f32 = 20.0;
#[allow(unused)]
pub const DIALOGUE_HEIGHT: f32 = 150.0;

pub enum Stage {
    PreAuth,

    #[allow(unused)]
    InMenu,
    InGame,
    Offline, // New stage for offline mode
}

pub struct GameState {
    stage: Stage,
    nc: NetClient,

    // Asset management
    asset_manager: AssetManager,

    // Player management
    players: Players,

    // Map
    map: Map,

    // Login state
    username: String,
    password: String,
    input_focus: InputField,
    auth_action: AuthAction, // New field to track the current auth action

    // Chat state
    is_chatting: bool,
    chat_input: String,
}

enum InputField {
    Username,
    Password,
    ActionType, // New field to track whether user wants to login or register
}

// Add a new enum to track the action type
enum AuthAction {
    Login,
    Register,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        Self::new_with_mode(ctx, false)
    }

    pub fn new_offline(ctx: &mut Context) -> Self {
        Self::new_with_mode(ctx, true)
    }

    pub fn new_with_map(ctx: &mut Context, map: Map) -> Self {
        Self::new_with_mode_and_map(ctx, false, map)
    }

    pub fn new_offline_with_map(ctx: &mut Context, map: Map) -> Self {
        Self::new_with_mode_and_map(ctx, true, map)
    }

    fn new_with_mode(ctx: &mut Context, offline_mode: bool) -> Self {
        // Create a default map
        let map = Map::new();
        Self::new_with_mode_and_map(ctx, offline_mode, map)
    }

    fn new_with_mode_and_map(ctx: &mut Context, offline_mode: bool, map: Map) -> Self {
        // Create network client based on mode
        let nc = if offline_mode {
            NetClient::new_offline()
        } else {
            NetClient::new()
        };

        // Create asset manager and load assets
        let mut asset_manager = AssetManager::new();

        // Load map assets
        asset_manager
            .load_assets(
                ctx,
                &[
                    ("floor", "/sprites/Files/Assets/Tilesets/Tileset_1/Floors/Floor(1)/floor_1(1).png"),
                    ("wall_middle", "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Walls/Walls(1)/wall(1)_mid.png"),
                    ("wall2", "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Side/wall_side_mid_left.png"),
                    ("wall3", "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Side/wall_side_mid_right.png"),
                    ("wall4", "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Top/wall_top_mid.png"),
                    ("wall5", "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Inner_Corner/wall_inner_corner_mid_left.png"),
                    ("wall6", "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Inner_Corner/wall_inner_corner_mid_rigth.png"),
                    ("skull", "/sprites/Files/Assets/Tilesets/Tileset_1/skull.png"),
                    ("door", "/sprites/Files/Assets/Tilesets/Tileset_1/Doors/doors_leaf_closed.png"),
                ],
            )
            .expect("Failed to load map assets");

        // Load assets for all character types
        let char_asset_names = vec!["Archer", "Knight", "Elf", "Lizard", "Wizzard"];
        for char_asset_name in char_asset_names {
            let expect = format!("Failed to load {} assets", char_asset_name);
            Self::load_character_assets(ctx, &mut asset_manager, char_asset_name).expect(&expect);
        }

        asset_manager.debug_print_loaded_assets();

        // Create player at starting position
        let start_pos = Position::new(
            (GRID_SIZE as f32 * 1.5) as i32,
            (GRID_SIZE as f32 * 1.5) as i32,
        );
        log::trace!(
            "Creating player at starting position: ({}, {})",
            start_pos.x,
            start_pos.y
        );
        let players = Players::new("Player".to_string(), start_pos);

        Self {
            stage: if offline_mode {
                Stage::Offline
            } else {
                // Start in PreAuth stage for online mode
                Stage::PreAuth
            },
            nc,
            asset_manager,
            players,
            map,
            username: String::new(),
            password: String::new(),
            input_focus: InputField::Username,
            auth_action: AuthAction::Login,
            is_chatting: false,
            chat_input: String::new(),
        }
    }

    // Helper method to load assets for a specific character type
    fn load_character_assets(
        ctx: &mut Context,
        asset_manager: &mut AssetManager,
        character: &str,
    ) -> GameResult<()> {
        let gender = "M"; // Using male characters for now
        let character_path = format!(
            "/sprites/Files/Assets/Heroes/{}/{}_{}",
            character, character, gender
        );

        log::trace!(
            "Loading character assets for {} from path: {}",
            character,
            character_path
        );

        // Load idle animations
        let idle_down_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );
        log::trace!("Loading idle down animation from: {}", idle_down_path);

        // Special handling for Archer to ensure assets are loaded correctly
        let load_result =
            asset_manager.load_asset(ctx, &format!("{}_idle_down", character), &idle_down_path);

        // If loading fails for Archer, try an alternative path
        if let Err(e) = load_result {
            if character == "Archer" {
                log::warn!(
                    "Failed to load Archer idle down animation: {}. Trying alternative path.",
                    e
                );
                // Try alternative path with correct casing
                let alt_path = format!(
                    "{}/archer_m_idle_anim/archer_m_idle_anim_f1.png",
                    character_path
                );
                asset_manager.load_asset(ctx, &format!("{}_idle_down", character), &alt_path)?;
            } else {
                return Err(e);
            }
        }

        let idle_up_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );
        log::trace!("Loading idle up animation from: {}", idle_up_path);

        // Special handling for Archer to ensure assets are loaded correctly
        let load_result =
            asset_manager.load_asset(ctx, &format!("{}_idle_up", character), &idle_up_path);

        // If loading fails for Archer, try an alternative path
        if let Err(e) = load_result {
            if character == "Archer" {
                log::warn!(
                    "Failed to load Archer idle up animation: {}. Trying alternative path.",
                    e
                );
                // Try alternative path with correct casing
                let alt_path = format!(
                    "{}/archer_m_idle_anim/archer_m_idle_anim_f1.png",
                    character_path
                );
                asset_manager.load_asset(ctx, &format!("{}_idle_up", character), &alt_path)?;
            } else {
                return Err(e);
            }
        }

        let idle_right_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );
        log::trace!("Loading idle right animation from: {}", idle_right_path);

        // Special handling for Archer to ensure assets are loaded correctly
        let load_result =
            asset_manager.load_asset(ctx, &format!("{}_idle_right", character), &idle_right_path);

        // If loading fails for Archer, try an alternative path
        if let Err(e) = load_result {
            if character == "Archer" {
                log::warn!(
                    "Failed to load Archer idle right animation: {}. Trying alternative path.",
                    e
                );
                // Try alternative path with correct casing
                let alt_path = format!(
                    "{}/archer_m_idle_anim/archer_m_idle_anim_f1.png",
                    character_path
                );
                asset_manager.load_asset(ctx, &format!("{}_idle_right", character), &alt_path)?;
            } else {
                return Err(e);
            }
        }

        // Add left-facing idle animation (using the same sprite as right for now)
        let idle_left_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );

        // Special handling for Archer to ensure assets are loaded correctly
        let load_result =
            asset_manager.load_asset(ctx, &format!("{}_idle_left", character), &idle_left_path);

        // If loading fails for Archer, try an alternative path
        if let Err(e) = load_result {
            if character == "Archer" {
                log::warn!(
                    "Failed to load Archer idle left animation: {}. Trying alternative path.",
                    e
                );
                // Try alternative path with correct casing
                let alt_path = format!(
                    "{}/archer_m_idle_anim/archer_m_idle_anim_f1.png",
                    character_path
                );
                asset_manager.load_asset(ctx, &format!("{}_idle_left", character), &alt_path)?;
            } else {
                return Err(e);
            }
        }

        // Load idle animation frames - use idle animations for all characters
        // since not all characters have sleep animations
        for i in 1..=4 {
            let anim_path = format!(
                "{}/{}_{}_idle_anim/{}_{}_idle_anim_f{}.png",
                character_path,
                character.to_lowercase(),
                gender.to_lowercase(),
                character.to_lowercase(),
                gender.to_lowercase(),
                i
            );

            // Special handling for Archer to ensure assets are loaded correctly
            let load_result =
                asset_manager.load_asset(ctx, &format!("{}_idle_{}", character, i), &anim_path);

            // If loading fails for Archer, try an alternative path
            if let Err(e) = load_result {
                if character == "Archer" {
                    log::warn!(
                        "Failed to load Archer idle animation frame {}: {}. Trying alternative path.",
                        i,
                        e
                    );
                    // Try alternative path with correct casing
                    let alt_path = format!(
                        "{}/archer_m_idle_anim/archer_m_idle_anim_f{}.png",
                        character_path, i
                    );
                    asset_manager.load_asset(
                        ctx,
                        &format!("{}_idle_{}", character, i),
                        &alt_path,
                    )?;
                } else {
                    return Err(e);
                }
            }
        }

        // Load run animations
        for i in 1..=4 {
            // Down direction
            let run_down_path = format!(
                "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                character_path,
                character.to_lowercase(),
                gender.to_lowercase(),
                character.to_lowercase(),
                gender.to_lowercase(),
                i
            );

            // Special handling for Archer to ensure assets are loaded correctly
            let load_result = asset_manager.load_asset(
                ctx,
                &format!("{}_run_down_{}", character, i),
                &run_down_path,
            );

            // If loading fails for Archer, try an alternative path
            if let Err(e) = load_result {
                if character == "Archer" {
                    log::warn!(
                        "Failed to load Archer run down animation frame {}: {}. Trying alternative path.",
                        i,
                        e
                    );
                    // Try alternative path with correct casing
                    let alt_path = format!(
                        "{}/archer_m_run_anim/archer_m_run_anim_f{}.png",
                        character_path, i
                    );
                    asset_manager.load_asset(
                        ctx,
                        &format!("{}_run_down_{}", character, i),
                        &alt_path,
                    )?;
                } else {
                    return Err(e);
                }
            }

            // Up direction
            let run_up_path = format!(
                "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                character_path,
                character.to_lowercase(),
                gender.to_lowercase(),
                character.to_lowercase(),
                gender.to_lowercase(),
                i
            );

            // Special handling for Archer to ensure assets are loaded correctly
            let load_result =
                asset_manager.load_asset(ctx, &format!("{}_run_up_{}", character, i), &run_up_path);

            // If loading fails for Archer, try an alternative path
            if let Err(e) = load_result {
                if character == "Archer" {
                    log::warn!(
                        "Failed to load Archer run up animation frame {}: {}. Trying alternative path.",
                        i,
                        e
                    );
                    // Try alternative path with correct casing
                    let alt_path = format!(
                        "{}/archer_m_run_anim/archer_m_run_anim_f{}.png",
                        character_path, i
                    );
                    asset_manager.load_asset(
                        ctx,
                        &format!("{}_run_up_{}", character, i),
                        &alt_path,
                    )?;
                } else {
                    return Err(e);
                }
            }

            // Right direction
            let run_right_path = format!(
                "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                character_path,
                character.to_lowercase(),
                gender.to_lowercase(),
                character.to_lowercase(),
                gender.to_lowercase(),
                i
            );

            // Special handling for Archer to ensure assets are loaded correctly
            let load_result = asset_manager.load_asset(
                ctx,
                &format!("{}_run_right_{}", character, i),
                &run_right_path,
            );

            // If loading fails for Archer, try an alternative path
            if let Err(e) = load_result {
                if character == "Archer" {
                    log::warn!(
                        "Failed to load Archer run right animation frame {}: {}. Trying alternative path.",
                        i,
                        e
                    );
                    // Try alternative path with correct casing
                    let alt_path = format!(
                        "{}/archer_m_run_anim/archer_m_run_anim_f{}.png",
                        character_path, i
                    );
                    asset_manager.load_asset(
                        ctx,
                        &format!("{}_run_right_{}", character, i),
                        &alt_path,
                    )?;
                } else {
                    return Err(e);
                }
            }

            // Left direction (using the same sprite as right for now)
            let run_left_path = format!(
                "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                character_path,
                character.to_lowercase(),
                gender.to_lowercase(),
                character.to_lowercase(),
                gender.to_lowercase(),
                i
            );

            // Special handling for Archer to ensure assets are loaded correctly
            let load_result = asset_manager.load_asset(
                ctx,
                &format!("{}_run_left_{}", character, i),
                &run_left_path,
            );

            // If loading fails for Archer, try an alternative path
            if let Err(e) = load_result {
                if character == "Archer" {
                    log::warn!(
                        "Failed to load Archer run left animation frame {}: {}. Trying alternative path.",
                        i,
                        e
                    );
                    // Try alternative path with correct casing
                    let alt_path = format!(
                        "{}/archer_m_run_anim/archer_m_run_anim_f{}.png",
                        character_path, i
                    );
                    asset_manager.load_asset(
                        ctx,
                        &format!("{}_run_left_{}", character, i),
                        &alt_path,
                    )?;
                } else {
                    return Err(e);
                }
            }
        }

        // Load fallback asset
        let fallback_path = format!("{}/{}_{}.png", character_path, character, gender);

        // Special handling for Archer to ensure assets are loaded correctly
        let load_result = asset_manager.load_asset(ctx, character, &fallback_path);

        // If loading fails for Archer, try an alternative path
        if let Err(e) = load_result {
            if character == "Archer" {
                log::warn!(
                    "Failed to load Archer fallback asset: {}. Trying alternative path.",
                    e
                );
                // Try alternative path with correct casing
                let alt_path = format!("{}/Archer_M.png", character_path);
                asset_manager.load_asset(ctx, character, &alt_path)?;
            } else {
                return Err(e);
            }
        }

        Ok(())
    }

    pub fn update(&mut self, ctx: &Context) -> GameResult<()> {
        match self.stage {
            Stage::PreAuth => self.update_pre_auth(ctx),
            Stage::InGame => self.update_in_game(ctx),
            Stage::Offline => self.update_offline(ctx),
            _ => {}
        }
        Ok(())
    }

    fn update_pre_auth(&mut self, ctx: &Context) {
        // Handle keyboard input for login fields
        self.handle_login_input(ctx);

        // Check for Enter key to submit login/registration
        if ctx.keyboard.is_key_just_pressed(KeyCode::Return)
            && !self.username.is_empty()
            && !self.password.is_empty()
        {
            match self.auth_action {
                AuthAction::Login => {
                    log::info!(
                        "Sending login for '{}' with password '{}'",
                        self.username,
                        self.password
                    );
                    let _ = self.nc.send(protocol::ClientToServer::Login(
                        self.username.clone(),
                        self.password.clone(),
                    ));
                }
                AuthAction::Register => {
                    log::info!(
                        "Sending registration for '{}' with password '{}'",
                        self.username,
                        self.password
                    );
                    let _ = self.nc.send(protocol::ClientToServer::Register(
                        self.username.clone(),
                        self.password.clone(),
                    ));
                }
            }
        }

        // Handle authentication
        let line = self.nc.recv();
        use crate::net::NCError::*;
        match line {
            Ok(ok) => {
                println!("{}", ok);
                // Check if login was successful and transition to InGame
                if ok.contains("Logged in") || ok.contains("Registered user") {
                    log::info!("Authentication successful, entering game.");

                    // Set the player's name to the username used for login
                    self.players.self_player.name = self.username.clone();
                    log::trace!("Set player name to: {}", self.username);

                    // Send the username to the server for identification
                    let user_event = protocol::ClientToServer::SetUsername(self.username.clone());
                    let _ = self.nc.send(user_event);

                    // Transition to InGame stage
                    self.stage = Stage::InGame;
                }
            }
            Err(err) => match err {
                NoNewData => {
                    // Auto-login for testing purposes has been removed
                    // We now have proper login/register UI
                }
                ConnectionError(e) => {
                    log::error!("Connection error: {}", e);
                }
                SendError => {
                    log::error!("Some random send error???")
                }
            },
        }
    }

    // New method to handle keyboard input for login fields
    fn handle_login_input(&mut self, ctx: &Context) {
        // Switch focus with Tab key
        if ctx.keyboard.is_key_just_pressed(KeyCode::Tab) {
            use InputField::*;
            self.input_focus = match self.input_focus {
                Username => Password,
                Password => ActionType,
                ActionType => Username,
            };
        }

        // Handle action type switching with arrow keys when ActionType is focused
        if matches!(self.input_focus, InputField::ActionType)
            && (ctx.keyboard.is_key_just_pressed(KeyCode::Left)
                || ctx.keyboard.is_key_just_pressed(KeyCode::Right))
        {
            self.auth_action = match self.auth_action {
                AuthAction::Login => AuthAction::Register,
                AuthAction::Register => AuthAction::Login,
            };
        }

        // Get the currently focused field if it's a text field
        if !matches!(self.input_focus, InputField::ActionType) {
            let current_field = match self.input_focus {
                InputField::Username => &mut self.username,
                InputField::Password => &mut self.password,
                _ => unreachable!(),
            };

            // Handle backspace - use KeyCode::Back instead of Backspace
            if ctx.keyboard.is_key_just_pressed(KeyCode::Back) && !current_field.is_empty() {
                current_field.pop();
            }

            // Handle text input
            // This is a simplified approach - in a real app, you'd use a proper text input system
            for key in [
                KeyCode::A,
                KeyCode::B,
                KeyCode::C,
                KeyCode::D,
                KeyCode::E,
                KeyCode::F,
                KeyCode::G,
                KeyCode::H,
                KeyCode::I,
                KeyCode::J,
                KeyCode::K,
                KeyCode::L,
                KeyCode::M,
                KeyCode::N,
                KeyCode::O,
                KeyCode::P,
                KeyCode::Q,
                KeyCode::R,
                KeyCode::S,
                KeyCode::T,
                KeyCode::U,
                KeyCode::V,
                KeyCode::W,
                KeyCode::X,
                KeyCode::Y,
                KeyCode::Z,
                KeyCode::Key1,
                KeyCode::Key2,
                KeyCode::Key3,
                KeyCode::Key4,
                KeyCode::Key5,
                KeyCode::Key6,
                KeyCode::Key7,
                KeyCode::Key8,
                KeyCode::Key9,
                KeyCode::Key0,
                KeyCode::Underline,
            ]
            .iter()
            {
                if ctx.keyboard.is_key_just_pressed(*key) {
                    let char_to_add = match key {
                        KeyCode::A => 'a',
                        KeyCode::B => 'b',
                        KeyCode::C => 'c',
                        KeyCode::D => 'd',
                        KeyCode::E => 'e',
                        KeyCode::F => 'f',
                        KeyCode::G => 'g',
                        KeyCode::H => 'h',
                        KeyCode::I => 'i',
                        KeyCode::J => 'j',
                        KeyCode::K => 'k',
                        KeyCode::L => 'l',
                        KeyCode::M => 'm',
                        KeyCode::N => 'n',
                        KeyCode::O => 'o',
                        KeyCode::P => 'p',
                        KeyCode::Q => 'q',
                        KeyCode::R => 'r',
                        KeyCode::S => 's',
                        KeyCode::T => 't',
                        KeyCode::U => 'u',
                        KeyCode::V => 'v',
                        KeyCode::W => 'w',
                        KeyCode::X => 'x',
                        KeyCode::Y => 'y',
                        KeyCode::Z => 'z',
                        KeyCode::Key1 => '1',
                        KeyCode::Key2 => '2',
                        KeyCode::Key3 => '3',
                        KeyCode::Key4 => '4',
                        KeyCode::Key5 => '5',
                        KeyCode::Key6 => '6',
                        KeyCode::Key7 => '7',
                        KeyCode::Key8 => '8',
                        KeyCode::Key9 => '9',
                        KeyCode::Key0 => '0',
                        KeyCode::Underline => '_',
                        _ => continue,
                    };

                    // Add the character to the current field
                    current_field.push(char_to_add);
                }
            }
        }
    }

    pub fn handle_text_input(
        &mut self,
        ctx: &mut Context,
        character: char,
    ) -> Result<(), ggez::GameError> {
        if self.is_chatting {
            self.chat_input.push(character);
            self.handle_chat_input(ctx);
        }
        Ok(())
    }

    fn update_in_game(&mut self, ctx: &Context) {
        // Check for chat toggle with tilde key
        if ctx.keyboard.is_key_just_pressed(KeyCode::Grave) {
            self.is_chatting = !self.is_chatting;
            if !self.is_chatting && !self.chat_input.is_empty() {
                // Send the chat message when exiting chat mode if there's a message
                let _ = self.nc.send(protocol::ClientToServer::ChatMessage(
                    self.chat_input.clone()
                ));

                // Also display the message for the local player
                self.players
                    .set_player_chat_message(&self.username, self.chat_input.clone());

                // Clear the chat input
                self.chat_input.clear();
            }
        }

        // Get the delta time for animations
        let delta_time = ctx.time.delta().as_secs_f32();

        // Handle chat input if in chat mode
        if self.is_chatting {
            self.handle_chat_input(ctx);

            // Even when chatting, we still need to update other players' animations
            // Create a no-movement state for the local player
            let no_movement = MovementState {
                is_moving: false,
                direction: self.players.self_player.direction,
                dx: 0,
                dy: 0,
            };

            // Update player animations but not position
            self.players
                .update(&no_movement, &self.map, GRID_SIZE, delta_time);
        } else {
            // Only process normal game input if not chatting
            // Get input
            let movement = input::handle_input(ctx);
            let key_press = input::handle_key_press(ctx);

            // Send movement to server
            input::send_movement_to_server(
                &mut self.nc,
                &movement,
                &self.username,
                &self.players.self_player.pos,
            );

            // Update player position
            self.players
                .update(&movement, &self.map, GRID_SIZE, delta_time);

            // Handle character switching
            if key_press.switch_character {
                self.players.switch_character();
            }

            // Check for door transitions
            let player_pos = self.players.self_player.pos;
            if let Some((new_room, new_x, new_y, facing)) =
                self.map
                    .check_door_transition(player_pos.x, player_pos.y, GRID_SIZE)
            {
                // Update the current room
                self.map.current_room = new_room;

                // Update player position to the new coordinates
                self.players.self_player.pos.x = new_x as i32;
                self.players.self_player.pos.y = new_y as i32;
                self.players.self_player.direction = facing;

                // Send the new position to the server
                self.send_absolute_position();
            }
        }

        // Process network messages regardless of chat state
        for _ in 0..3 {
            self.process_network_messages();
        }
    }

    // Helper method to send the player's absolute position to the server
    fn send_absolute_position(&mut self) {
        // Send username for identification
        let username_event = protocol::ClientToServer::SetUsername(self.username.clone());
        let _ = self.nc.send(username_event);

        // Send current facing direction
        let facing_event =
            protocol::ClientToServer::AttemptPlayerFacingChange(self.players.self_player.direction);
        let _ = self.nc.send(facing_event);

        // Send absolute position
        let move_event = protocol::ClientToServer::SetPosition(
            self.players.self_player.pos.x,
            self.players.self_player.pos.y,
        );
        let _ = self.nc.send(move_event);

        // Log the position being sent
        log::trace!(
            "Sending absolute position: ({}, {})",
            self.players.self_player.pos.x,
            self.players.self_player.pos.y
        );
    }

    // Method to process network messages for other players
    fn process_network_messages(&mut self) {
        // Only process messages if we're not in offline mode
        if self.nc.is_offline() {
            return;
        }

        // Process all available messages in a loop
        let mut message_count = 0;
        let max_messages_per_frame = 10; // Limit to prevent infinite loops

        loop {
            // Try to receive a message from the server
            match self.nc.recv() {
                Ok(message) => {
                    message_count += 1;

                    // Skip "Invalid protocol" messages
                    if message.contains("Invalid protocol") {
                        continue;
                    }

                    // Log raw messages for debugging
                    log::trace!("Raw server message: {}", message);

                    // Parse the message to see if it's a player update
                    if let Some(server_message) = self.nc.parse_server_message(&message) {
                        match server_message {
                            protocol::ServerToClient::PlayerJoined(username, position, facing) => {
                                // Skip if this is our own username
                                if username == self.username {
                                    log::info!("Skipping own player joined message: {}", username);
                                    continue;
                                }

                                log::info!(
                                    "Player joined: {} at ({}, {})",
                                    username,
                                    position.x,
                                    position.y
                                );
                                self.players
                                    .add_or_update_player(username, position, facing);

                                // Debug print all players
                                self.players.debug_print_players();
                            }
                            protocol::ServerToClient::PlayerLeft(username) => {
                                log::info!("Player left: {}", username);
                                self.players.remove_player(&username);

                                // Debug print all players
                                self.players.debug_print_players();
                            }
                            protocol::ServerToClient::PlayerMoved(username, position, facing) => {
                                // Skip if this is our own username
                                if username == self.username {
                                    log::trace!("Skipping own player moved message: {}", username);
                                    continue;
                                }

                                // Only log position updates if the position is not (0,0)
                                // This is to avoid logging facing-only updates
                                if position.x != 0 || position.y != 0 {
                                    log::trace!(
                                        "Player moved: {} to ({}, {})",
                                        username,
                                        position.x,
                                        position.y
                                    );
                                }

                                // Update the player's position and facing
                                self.players
                                    .update_player_position(&username, position, facing);
                            }
                            protocol::ServerToClient::ChatMessage(username, message) => {
                                log::info!("Chat message from {}: {}", username, message);

                                // Display the chat message above the player
                                self.players.set_player_chat_message(&username, message);
                            }
                            _ => {
                                log::info!("Received other message type: {:?}", server_message);
                            }
                        }
                    } else if message.contains("player_joined") || message.contains("player_moved")
                    {
                        // Try to extract the username and position manually
                        let parts: Vec<&str> = message.split_whitespace().collect();
                        log::trace!("Message parts: {:?}", parts);

                        if parts.len() >= 5 {
                            if let Some(pos) = parts
                                .iter()
                                .position(|&p| p == "player_joined" || p == "player_moved")
                            {
                                if pos + 4 < parts.len() {
                                    let cmd = parts[pos];
                                    let username = parts[pos + 1].to_string();

                                    // Skip if this is our own username
                                    if username == self.username {
                                        log::info!("Skipping own player message: {}", username);
                                        continue;
                                    }

                                    if let (Ok(x), Ok(y)) = (
                                        parts[pos + 2].parse::<i32>(),
                                        parts[pos + 3].parse::<i32>(),
                                    ) {
                                        let facing_str = parts[pos + 4];
                                        let facing = protocol::Facing::from_str(facing_str)
                                            .expect("string not valid");

                                        let position = protocol::Position::new(x, y);

                                        if cmd == "player_joined" {
                                            log::trace!(
                                                "Manual parse - Player joined: {} at ({}, {})",
                                                username,
                                                x,
                                                y
                                            );
                                            self.players
                                                .add_or_update_player(username, position, facing);

                                            // Debug print all players
                                            self.players.debug_print_players();
                                        } else {
                                            log::trace!(
                                                "Manual parse - Player moved: {} to ({}, {})",
                                                username,
                                                x,
                                                y
                                            );
                                            self.players.update_player_position(
                                                &username, position, facing,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Break if we've processed too many messages in one frame
                    if message_count >= max_messages_per_frame {
                        break;
                    }
                }
                Err(_) => {
                    // No more messages to process
                    break;
                }
            }
        }
    }

    fn update_offline(&mut self, ctx: &Context) {
        // Check for chat toggle with tilde key
        if ctx.keyboard.is_key_just_pressed(KeyCode::Grave) {
            self.is_chatting = !self.is_chatting;
            if !self.is_chatting && !self.chat_input.is_empty() {
                // Display the message for the local player
                self.players
                    .set_player_chat_message(&self.username, self.chat_input.clone());

                // Clear the chat input
                self.chat_input.clear();
            }
        }

        // Get the delta time for animations
        let delta_time = ctx.time.delta().as_secs_f32();

        // Handle chat input if in chat mode
        if self.is_chatting {
            self.handle_chat_input(ctx);

            // Even when chatting, we still need to update other players' animations
            // Create a no-movement state for the local player
            let no_movement = MovementState {
                is_moving: false,
                direction: self.players.self_player.direction,
                dx: 0,
                dy: 0,
            };

            // Update player animations but not position
            self.players
                .update(&no_movement, &self.map, GRID_SIZE, delta_time);
        } else {
            // Handle input
            let movement = input::handle_input(ctx);
            let key_press = input::handle_key_press(ctx);

            // Update player position
            self.players
                .update(&movement, &self.map, GRID_SIZE, delta_time);

            // Handle character switching
            if key_press.switch_character {
                self.players.switch_character();
            }

            // Check for door transitions
            let player_pos = self.players.self_player.pos;
            if let Some((new_room, door_x, door_y, facing)) =
                self.map
                    .check_door_transition(player_pos.x, player_pos.y, GRID_SIZE)
            {
                // Update the current room
                self.map.current_room = new_room;

                // Update player position to the new coordinates
                self.players.self_player.pos.x = door_x as i32;
                self.players.self_player.pos.y = door_y as i32;
                self.players.self_player.direction = facing;
            }
        }

        // Simulate other players in offline mode
        self.simulate_other_players(delta_time);
    }

    // New method to simulate other players in offline mode
    fn simulate_other_players(&mut self, delta_time: f32) {
        // Only add simulated players if we don't have any yet
        if self.players.other_players.is_empty() {
            // Add a simulated player that moves around
            let start_pos = protocol::Position::new(
                (GRID_SIZE as f32 * 3.5) as i32,
                (GRID_SIZE as f32 * 3.5) as i32,
            );
            self.players.add_or_update_player(
                "SimPlayer".to_string(),
                start_pos,
                protocol::Facing::South,
            );

            // Add another simulated player that stays still
            let start_pos2 = protocol::Position::new(
                (GRID_SIZE as f32 * 5.5) as i32,
                (GRID_SIZE as f32 * 2.5) as i32,
            );
            self.players.add_or_update_player(
                "StaticPlayer".to_string(),
                start_pos2,
                protocol::Facing::East,
            );
        }

        // Make the simulated player move in a pattern
        static mut MOVE_TIMER: f32 = 0.0;
        static mut DIRECTION: i32 = 0;

        unsafe {
            MOVE_TIMER += delta_time;

            // Change direction every 2 seconds
            if MOVE_TIMER > 2.0 {
                MOVE_TIMER = 0.0;
                DIRECTION = (DIRECTION + 1) % 4;

                // Find the simulated player
                for player in &mut self.players.other_players {
                    if player.name == "SimPlayer" {
                        // Update the direction
                        use protocol::Facing::*;
                        player.direction = match DIRECTION {
                            0 => North,
                            1 => East,
                            2 => South,
                            3 => West,
                            _ => South,
                        };

                        // Move the player in that direction
                        let (dx, dy) = match player.direction {
                            North => (0, -1),
                            East => (1, 0),
                            South => (0, 1),
                            West => (-1, 0),
                        };

                        player.pos.x += dx * GRID_SIZE;
                        player.pos.y += dy * GRID_SIZE;
                        player.is_moving = true;

                        // Keep the player within bounds
                        player.pos.x = player.pos.x.max(GRID_SIZE).min(GRID_SIZE * 10);
                        player.pos.y = player.pos.y.max(GRID_SIZE).min(GRID_SIZE * 10);

                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        use Stage::*;
        match self.stage {
            PreAuth => self.draw_pre_auth(ctx, &mut canvas),
            InMenu => {}
            InGame => self.draw_in_game(ctx, &mut canvas),
            Offline => self.draw_offline(ctx, &mut canvas), // Use the same drawing code for offline mode
        }

        // Draw offline mode indicator if in offline mode
        if let Stage::Offline = self.stage {
            self.draw_offline_indicator(ctx, &mut canvas);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn draw_pre_auth(&self, ctx: &Context, canvas: &mut graphics::Canvas) {
        // Draw a simple login screen
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;

        // Set screen coordinates for UI elements
        canvas.set_screen_coordinates(Rect::new(0.0, 0.0, screen_width, screen_height));

        // Draw background - use a filled rectangle instead of clear
        let bg_color = Color::new(0.1, 0.1, 0.2, 1.0);
        let bg_rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(0.0, 0.0, screen_width, screen_height),
            bg_color,
        )
        .unwrap();
        canvas.draw(&bg_rect, DrawParam::default());

        // Draw title
        let title_text = Text::new("Login / Register");
        let title_dimensions = title_text.dimensions(ctx).unwrap();
        let title_width = title_dimensions.w;

        canvas.draw(
            &title_text,
            DrawParam::default()
                .dest([
                    screen_width / 2.0 - title_width / 2.0,
                    screen_height / 2.0 - 100.0,
                ])
                .color(Color::WHITE),
        );

        // Draw username field
        let username_label = Text::new("Username:");
        canvas.draw(
            &username_label,
            DrawParam::default()
                .dest([screen_width / 2.0 - 150.0, screen_height / 2.0 - 40.0])
                .color(Color::WHITE),
        );

        // Draw username input box
        let username_box_color = if matches!(self.input_focus, InputField::Username) {
            Color::new(0.3, 0.3, 0.6, 1.0) // Highlighted
        } else {
            Color::new(0.2, 0.2, 0.4, 1.0) // Normal
        };

        let username_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                screen_width / 2.0 - 50.0,
                screen_height / 2.0 - 45.0,
                200.0,
                30.0,
            ),
            username_box_color,
        )
        .unwrap();
        canvas.draw(&username_box, DrawParam::default());

        // Draw username text
        let username_text = Text::new(&self.username);
        canvas.draw(
            &username_text,
            DrawParam::default()
                .dest([screen_width / 2.0 - 40.0, screen_height / 2.0 - 40.0])
                .color(Color::WHITE),
        );

        // Draw password field
        let password_label = Text::new("Password:");
        canvas.draw(
            &password_label,
            DrawParam::default()
                .dest([screen_width / 2.0 - 150.0, screen_height / 2.0])
                .color(Color::WHITE),
        );

        // Draw password input box
        let password_box_color = if matches!(self.input_focus, InputField::Password) {
            Color::new(0.3, 0.3, 0.6, 1.0) // Highlighted
        } else {
            Color::new(0.2, 0.2, 0.4, 1.0) // Normal
        };

        let password_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                screen_width / 2.0 - 50.0,
                screen_height / 2.0 - 5.0,
                200.0,
                30.0,
            ),
            password_box_color,
        )
        .unwrap();
        canvas.draw(&password_box, DrawParam::default());

        // Draw password text (masked with asterisks)
        let masked_password = "*".repeat(self.password.len());
        let password_text = Text::new(&masked_password);
        canvas.draw(
            &password_text,
            DrawParam::default()
                .dest([screen_width / 2.0 - 40.0, screen_height / 2.0])
                .color(Color::WHITE),
        );

        // Draw action type selection (Login/Register)
        let action_label = Text::new("Action:");
        canvas.draw(
            &action_label,
            DrawParam::default()
                .dest([screen_width / 2.0 - 150.0, screen_height / 2.0 + 40.0])
                .color(Color::WHITE),
        );

        // Draw action type selection box
        let action_box_color = if matches!(self.input_focus, InputField::ActionType) {
            Color::new(0.3, 0.3, 0.6, 1.0) // Highlighted
        } else {
            Color::new(0.2, 0.2, 0.4, 1.0) // Normal
        };

        let action_box = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(
                screen_width / 2.0 - 50.0,
                screen_height / 2.0 + 35.0,
                200.0,
                30.0,
            ),
            action_box_color,
        )
        .unwrap();
        canvas.draw(&action_box, DrawParam::default());

        // Draw current action type
        let action_text = match self.auth_action {
            AuthAction::Login => "Login",
            AuthAction::Register => "Register",
        };
        let action_text = Text::new(action_text);
        canvas.draw(
            &action_text,
            DrawParam::default()
                .dest([screen_width / 2.0 - 40.0, screen_height / 2.0 + 40.0])
                .color(Color::WHITE),
        );

        // Draw instructions
        let instructions_text = Text::new("Press Enter to submit");
        let instructions_dimensions = instructions_text.dimensions(ctx).unwrap();
        let instructions_width = instructions_dimensions.w;

        canvas.draw(
            &instructions_text,
            DrawParam::default()
                .dest([
                    screen_width / 2.0 - instructions_width / 2.0,
                    screen_height / 2.0 + 80.0,
                ])
                .color(Color::YELLOW),
        );

        // Draw tab instruction
        let tab_text = Text::new("Press Tab to switch fields");
        let tab_dimensions = tab_text.dimensions(ctx).unwrap();
        let tab_width = tab_dimensions.w;

        canvas.draw(
            &tab_text,
            DrawParam::default()
                .dest([
                    screen_width / 2.0 - tab_width / 2.0,
                    screen_height / 2.0 + 110.0,
                ])
                .color(Color::YELLOW),
        );

        // Draw arrow key instruction for action type
        let arrow_text = Text::new("Use Left/Right arrows to change action type");
        let arrow_dimensions = arrow_text.dimensions(ctx).unwrap();
        let arrow_width = arrow_dimensions.w;

        canvas.draw(
            &arrow_text,
            DrawParam::default()
                .dest([
                    screen_width / 2.0 - arrow_width / 2.0,
                    screen_height / 2.0 + 140.0,
                ])
                .color(Color::YELLOW),
        );
    }

    fn draw_in_game(&self, ctx: &Context, canvas: &mut graphics::Canvas) {
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;

        let zoomed_width = screen_width / CAMERA_ZOOM;
        let zoomed_height = screen_height / CAMERA_ZOOM;

        // Center the camera on the player's center (not top-left corner)
        // Add half the player size to center on the player sprite
        let player_center_x = self.players.self_player.pos.x + input::PLAYER_SIZE / 2;
        let player_center_y = self.players.self_player.pos.y + input::PLAYER_SIZE / 2;

        let camera_x = player_center_x - zoomed_width as i32 / 2;
        let camera_y = player_center_y - zoomed_height as i32 / 2;

        // Set the camera view
        canvas.set_screen_coordinates(Rect::new(
            camera_x as f32,
            camera_y as f32,
            zoomed_width,
            zoomed_height,
        ));

        // Draw the map first (so it's behind the player)
        self.map
            .draw(ctx, canvas, &self.asset_manager, GRID_SIZE)
            .unwrap();

        // Draw all players
        self.players.draw(ctx, canvas, &self.asset_manager).unwrap();

        // Draw position info for debugging - fixed to the camera view
        let pos_text = Text::new(format!(
            "Pos: ({:.1}, {:.1}) - Room: {}",
            self.players.self_player.pos.x, self.players.self_player.pos.y, self.map.current_room
        ));

        // Draw UI elements in screen coordinates by adding the camera position
        canvas.draw(
            &pos_text,
            DrawParam::default()
                .dest([(camera_x + 10) as f32, (camera_y + 10) as f32])
                .color(Color::WHITE),
        );

        // Draw chat input box if in chat mode
        if self.is_chatting {
            // Switch to screen coordinates for UI elements
            canvas.set_screen_coordinates(Rect::new(0.0, 0.0, screen_width, screen_height));

            // Draw chat input background
            let chat_bg = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(10.0, screen_height - 40.0, screen_width - 20.0, 30.0),
                Color::new(0.0, 0.0, 0.0, 0.7), // Semi-transparent black
            )
            .unwrap();
            canvas.draw(&chat_bg, DrawParam::default());

            // Draw chat input text
            let chat_text = Text::new(format!("Chat: {}", self.chat_input));
            canvas.draw(
                &chat_text,
                DrawParam::default()
                    .dest([20.0, screen_height - 35.0])
                    .color(Color::WHITE),
            );

            // Draw chat instructions
            let chat_instructions = Text::new("Press Enter to send, Esc to cancel");
            canvas.draw(
                &chat_instructions,
                DrawParam::default()
                    .dest([20.0, screen_height - 60.0])
                    .color(Color::YELLOW),
            );

            // Reset to game coordinates
            canvas.set_screen_coordinates(Rect::new(
                camera_x as f32,
                camera_y as f32,
                zoomed_width,
                zoomed_height,
            ));
        } else {
            // Draw chat hint when not in chat mode
            // Switch to screen coordinates for UI elements
            canvas.set_screen_coordinates(Rect::new(0.0, 0.0, screen_width, screen_height));

            let chat_hint = Text::new("Press ~ to chat");
            canvas.draw(
                &chat_hint,
                DrawParam::default()
                    .dest([20.0, screen_height - 30.0])
                    .color(Color::YELLOW),
            );

            // Reset to game coordinates
            canvas.set_screen_coordinates(Rect::new(
                camera_x as f32,
                camera_y as f32,
                zoomed_width,
                zoomed_height,
            ));
        }
    }

    fn draw_offline(&self, ctx: &Context, canvas: &mut graphics::Canvas) {
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;

        let zoomed_width = screen_width / CAMERA_ZOOM;
        let zoomed_height = screen_height / CAMERA_ZOOM;

        // Center the camera on the player's center (not top-left corner)
        // Add half the player size to center on the player sprite
        let player_center_x = self.players.self_player.pos.x + input::PLAYER_SIZE / 2;
        let player_center_y = self.players.self_player.pos.y + input::PLAYER_SIZE / 2;

        let camera_x = player_center_x - zoomed_width as i32 / 2;
        let camera_y = player_center_y - zoomed_height as i32 / 2;

        // Set the camera view
        canvas.set_screen_coordinates(Rect::new(
            camera_x as f32,
            camera_y as f32,
            zoomed_width,
            zoomed_height,
        ));

        // Draw the map first (so it's behind the player)
        self.map
            .draw(ctx, canvas, &self.asset_manager, GRID_SIZE)
            .unwrap();

        // Draw all players
        self.players.draw(ctx, canvas, &self.asset_manager).unwrap();

        // Draw position info for debugging - fixed to the camera view
        let pos_text = Text::new(format!(
            "Pos: ({:.1}, {:.1}) - Room: {}",
            self.players.self_player.pos.x, self.players.self_player.pos.y, self.map.current_room
        ));

        // Draw UI elements in screen coordinates by adding the camera position
        canvas.draw(
            &pos_text,
            DrawParam::default()
                .dest([(camera_x + 10) as f32, (camera_y + 10) as f32])
                .color(Color::WHITE),
        );

        // Draw chat input box if in chat mode
        if self.is_chatting {
            // Switch to screen coordinates for UI elements
            canvas.set_screen_coordinates(Rect::new(0.0, 0.0, screen_width, screen_height));

            // Draw chat input background
            let chat_bg = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                Rect::new(10.0, screen_height - 40.0, screen_width - 20.0, 30.0),
                Color::new(0.0, 0.0, 0.0, 0.7), // Semi-transparent black
            )
            .unwrap();
            canvas.draw(&chat_bg, DrawParam::default());

            // Draw chat input text
            let chat_text = Text::new(format!("Chat: {}", self.chat_input));
            canvas.draw(
                &chat_text,
                DrawParam::default()
                    .dest([20.0, screen_height - 35.0])
                    .color(Color::WHITE),
            );

            // Draw chat instructions
            let chat_instructions = Text::new("Press Enter to send, Esc to cancel");
            canvas.draw(
                &chat_instructions,
                DrawParam::default()
                    .dest([20.0, screen_height - 60.0])
                    .color(Color::YELLOW),
            );

            // Reset to game coordinates
            canvas.set_screen_coordinates(Rect::new(
                camera_x as f32,
                camera_y as f32,
                zoomed_width,
                zoomed_height,
            ));
        } else {
            // Draw chat hint when not in chat mode
            // Switch to screen coordinates for UI elements
            canvas.set_screen_coordinates(Rect::new(0.0, 0.0, screen_width, screen_height));

            let chat_hint = Text::new("Press ~ to chat");
            canvas.draw(
                &chat_hint,
                DrawParam::default()
                    .dest([20.0, screen_height - 30.0])
                    .color(Color::YELLOW),
            );

            // Reset to game coordinates
            canvas.set_screen_coordinates(Rect::new(
                camera_x as f32,
                camera_y as f32,
                zoomed_width,
                zoomed_height,
            ));
        }

        // Draw offline mode indicator
        self.draw_offline_indicator(ctx, canvas);
    }

    fn draw_offline_indicator(&self, ctx: &Context, canvas: &mut graphics::Canvas) {
        // Draw offline mode indicator in the top-right corner
        let screen_width = ctx.gfx.window().inner_size().width as f32;

        let text = Text::new("OFFLINE MODE");
        let text_pos = [screen_width - 120.0, 10.0];
        canvas.draw(
            &text,
            DrawParam::default().dest(text_pos).color(Color::YELLOW),
        );
    }

    // Method to handle chat input
    fn handle_chat_input(&mut self, ctx: &Context) {
        // Handle Enter key to send the message
        if ctx.keyboard.is_key_just_pressed(KeyCode::Return) {
            if !self.chat_input.is_empty() {
                // In online mode, send the chat message to the server
                if !self.nc.is_offline() {
                    let chat_msg = format!("chat {}\r\n", self.chat_input);
                    let _ = self.nc.send(protocol::ClientToServer::ChatMessage(
                        self.chat_input.clone()
                    ));
                }

                // Always display the message for the local player
                self.players
                    .set_player_chat_message(&self.username, self.chat_input.clone());

                // Clear the chat input and exit chat mode
                self.chat_input.clear();
                self.is_chatting = false;
            } else {
                // If input is empty, just exit chat mode
                self.is_chatting = false;
            }
            return;
        }

        // Handle Escape key to cancel chat
        if ctx.keyboard.is_key_just_pressed(KeyCode::Escape) {
            self.chat_input.clear();
            self.is_chatting = false;
            return;
        }

        // Handle backspace
        if ctx.keyboard.is_key_just_pressed(KeyCode::Back) && !self.chat_input.is_empty() {
            self.chat_input.pop();
        }
    }
}
