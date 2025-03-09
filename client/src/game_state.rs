use ggez::{
    Context, GameResult,
    graphics::{self, Color, DrawParam, Rect, Text},
};
use protocol::{ClientToServer, Position};

use crate::{
    assets::AssetManager,
    input::{self, handle_key_press},
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
        let mut nc = if offline_mode {
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
        let start_pos = Position::new((GRID_SIZE as f32 * 1.5) as i32, (GRID_SIZE as f32 * 1.5) as i32);
        log::info!("Creating player at starting position: ({}, {})", start_pos.x, start_pos.y);
        let players = Players::new("Player".to_string(), start_pos);

        Self {
            stage: if offline_mode { 
                Stage::Offline 
            } else { 
                // Send registration/login command if online
                let event = ClientToServer::Register("xyz".to_string(), "123".to_string());
                let _ = nc.send(event);
                // Wait a bit for server response
                std::thread::sleep(std::time::Duration::from_millis(100));
                Stage::PreAuth 
            },
            nc,
            asset_manager,
            players,
            map,
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
        
        log::info!("Loading character assets for {} from path: {}", character, character_path);

        // Load idle animations
        let idle_down_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );
        log::info!("Loading idle down animation from: {}", idle_down_path);
        
        asset_manager.load_asset(
            ctx,
            &format!("{}_idle_down", character),
            &idle_down_path,
        )?;

        let idle_up_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );
        log::info!("Loading idle up animation from: {}", idle_up_path);
        
        asset_manager.load_asset(
            ctx,
            &format!("{}_idle_up", character),
            &idle_up_path,
        )?;

        let idle_right_path = format!(
            "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
            character_path,
            character.to_lowercase(),
            gender.to_lowercase(),
            character.to_lowercase(),
            gender.to_lowercase()
        );
        log::info!("Loading idle right animation from: {}", idle_right_path);
        
        asset_manager.load_asset(
            ctx,
            &format!("{}_idle_right", character),
            &idle_right_path,
        )?;

        // Add left-facing idle animation (using the same sprite as right for now)
        asset_manager.load_asset(
            ctx,
            &format!("{}_idle_left", character),
            &format!(
                "{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png",
                character_path,
                character.to_lowercase(),
                gender.to_lowercase(),
                character.to_lowercase(),
                gender.to_lowercase()
            ),
        )?;

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

            asset_manager.load_asset(ctx, &format!("{}_idle_{}", character, i), &anim_path)?;
        }

        // Load run animations
        for i in 1..=4 {
            // Down direction
            asset_manager.load_asset(
                ctx,
                &format!("{}_run_down_{}", character, i),
                &format!(
                    "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                    character_path,
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    i
                ),
            )?;

            // Up direction
            asset_manager.load_asset(
                ctx,
                &format!("{}_run_up_{}", character, i),
                &format!(
                    "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                    character_path,
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    i
                ),
            )?;

            // Right direction
            asset_manager.load_asset(
                ctx,
                &format!("{}_run_right_{}", character, i),
                &format!(
                    "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                    character_path,
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    i
                ),
            )?;
            
            // Left direction (using the same sprite as right for now)
            asset_manager.load_asset(
                ctx,
                &format!("{}_run_left_{}", character, i),
                &format!(
                    "{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png",
                    character_path,
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    character.to_lowercase(),
                    gender.to_lowercase(),
                    i
                ),
            )?;
        }

        // Load fallback asset
        asset_manager.load_asset(
            ctx,
            character,
            &format!("{}/{}_{}.png", character_path, character, gender),
        )?;

        Ok(())
    }

    pub fn update(&mut self, ctx: &Context) -> GameResult<()> {
        use Stage::*;
        match self.stage {
            PreAuth => self.update_pre_auth(),
            InMenu => {}
            InGame => self.update_in_game(ctx),
            Offline => self.update_offline(ctx),
        }

        Ok(())
    }

    fn update_pre_auth(&mut self) {
        // Handle authentication
        let line = self.nc.recv();
        use crate::net::NCError::*;
        match line {
            Ok(ok) => {
                println!("{}", ok);
                // Check if login was successful and transition to InGame
                if ok.contains("Logged in") || ok.contains("Registered user") {
                    log::info!("Authentication successful, entering game.");
                    self.stage = Stage::InGame;
                }
            }
            Err(err) => match err {
                NoNewData => {}
                ConnectionError(e) => {
                    log::error!("Connection error: {}", e);
                }
                SendError => {
                    log::error!("Some random send error???")
                }
            },
        }
    }

    fn update_in_game(&mut self, ctx: &Context) {
        // Get input
        let movement = input::handle_input(ctx);
        let key_press = input::handle_key_press(ctx);

        // Send movement to server
        input::send_movement_to_server(&mut self.nc, &movement);

        // Update player position
        let delta_time = ctx.time.delta().as_secs_f32();
        self.players.update(&movement, &self.map, GRID_SIZE, delta_time);

        // Handle character switching
        if key_press.switch_character {
            self.players.switch_character();
        }

        // Check for door transitions
        let player_pos = self.players.self_player.pos;
        if let Some((new_room, door_x, door_y, facing)) = self.map.check_door_transition(player_pos.x, player_pos.y, GRID_SIZE) {
            // Update the current room
            self.map.current_room = new_room;
            
            // Calculate the new position based on the door and facing direction
            let grid_x = door_x as i32;
            let grid_y = door_y as i32;
            
            // Apply a larger offset to ensure the player doesn't get stuck in the door
            let offset = 2; // Use 2 grid cells of offset to prevent re-triggering
            
            let (new_x, new_y) = match facing {
                protocol::Facing::North => (grid_x * GRID_SIZE, grid_y * GRID_SIZE - offset * GRID_SIZE),
                protocol::Facing::South => (grid_x * GRID_SIZE, grid_y * GRID_SIZE + offset * GRID_SIZE),
                protocol::Facing::East => (grid_x * GRID_SIZE + offset * GRID_SIZE, grid_y * GRID_SIZE),
                protocol::Facing::West => (grid_x * GRID_SIZE - offset * GRID_SIZE, grid_y * GRID_SIZE),
            };
            
            // Update player position and direction
            self.players.self_player.pos.x = new_x;
            self.players.self_player.pos.y = new_y;
            self.players.self_player.direction = facing;
            
            log::info!("Transitioned to room {} at position ({}, {})", new_room, new_x, new_y);
        }
    }

    fn update_offline(&mut self, ctx: &Context) {
        // Handle input
        let movement = input::handle_input(ctx);
        let key_press = input::handle_key_press(ctx);

        // Update player position
        let delta_time = ctx.time.delta().as_secs_f32();
        self.players.update(&movement, &self.map, GRID_SIZE, delta_time);

        // Log player position for debugging
        log::info!(
            "Player position: ({}, {}), Room: {}", 
            self.players.self_player.pos.x, 
            self.players.self_player.pos.y,
            self.map.current_room
        );

        // Check for character switch
        if key_press.switch_character {
            self.players.switch_character();
        }

        // Check for door transitions
        if let Some((new_room, door_x, door_y, facing)) = self.map.check_door_transition(
            self.players.self_player.pos.x,
            self.players.self_player.pos.y,
            GRID_SIZE,
        ) {
            // Update the current room
            self.map.current_room = new_room;
            
            // Calculate the new position based on the door and facing direction
            let grid_x = door_x as i32;
            let grid_y = door_y as i32;
            
            // Apply a larger offset to ensure the player doesn't get stuck in the door
            let offset = 2; // Use 2 grid cells of offset to prevent re-triggering
            
            let (new_x, new_y) = match facing {
                protocol::Facing::North => (grid_x * GRID_SIZE, grid_y * GRID_SIZE - offset * GRID_SIZE),
                protocol::Facing::South => (grid_x * GRID_SIZE, grid_y * GRID_SIZE + offset * GRID_SIZE),
                protocol::Facing::East => (grid_x * GRID_SIZE + offset * GRID_SIZE, grid_y * GRID_SIZE),
                protocol::Facing::West => (grid_x * GRID_SIZE - offset * GRID_SIZE, grid_y * GRID_SIZE),
            };
            
            // Update player position and direction
            self.players.self_player.pos.x = new_x;
            self.players.self_player.pos.y = new_y;
            self.players.self_player.direction = facing;
            
            log::info!("Transitioned to room {} at position ({}, {})", new_room, new_x, new_y);
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        use Stage::*;
        match self.stage {
            PreAuth => self.draw_pre_auth(ctx, &mut canvas),
            InMenu => {}
            InGame => self.draw_in_game(ctx, &mut canvas),
            Offline => self.draw_in_game(ctx, &mut canvas), // Use the same drawing code for offline mode
        }

        // Draw offline mode indicator if in offline mode
        if let Stage::Offline = self.stage {
            self.draw_offline_indicator(ctx, &mut canvas);
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn draw_pre_auth(&self, ctx: &Context, canvas: &mut graphics::Canvas) {
        // Draw login/authentication screen
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;

        // Draw text for login screen
        let text = Text::new("Authenticating...");
        let text_pos = [screen_width / 2.0 - 50.0, screen_height / 2.0];
        canvas.draw(
            &text,
            DrawParam::default().dest(text_pos).color(Color::WHITE),
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
        canvas.set_screen_coordinates(Rect::new(camera_x as f32, camera_y as f32, zoomed_width, zoomed_height));

        // Draw the map first (so it's behind the player)
        self.map
            .draw(ctx, canvas, &self.asset_manager, GRID_SIZE)
            .unwrap();

        // Draw all players
        self.players.draw(canvas, &self.asset_manager).unwrap();

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
}
