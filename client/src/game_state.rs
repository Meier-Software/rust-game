use ggez::{
    Context, GameResult,
    graphics::{self, Color, DrawParam, Rect, Text},
};
use protocol::Position;

use crate::{
    assets::AssetManager,
    input::{self, handle_key_press},
    map::Map,
    net::NetClient,
    player::{Players, CharacterType},
};

// Constants
pub const GRID_SIZE: f32 = 16.0;
pub const CAMERA_ZOOM: f32 = 4.0;
pub const DIALOGUE_PADDING: f32 = 20.0;
pub const DIALOGUE_HEIGHT: f32 = 150.0;

pub enum Stage {
    PreAuth,
    InMenu,
    InGame,
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
        let mut nc = NetClient::new();
        let mut asset_manager = AssetManager::new();

        // Load assets for all character types
        Self::load_character_assets(ctx, &mut asset_manager, "Knight").expect("Failed to load Knight assets");
        Self::load_character_assets(ctx, &mut asset_manager, "Archer").expect("Failed to load Archer assets");
        Self::load_character_assets(ctx, &mut asset_manager, "Elf").expect("Failed to load Elf assets");
        Self::load_character_assets(ctx, &mut asset_manager, "Lizard").expect("Failed to load Lizard assets");
        Self::load_character_assets(ctx, &mut asset_manager, "Wizzard").expect("Failed to load Wizard assets");
        
        // Load wall assets and other assets
        let wall_assets = [
            // Wall assets - using the available wall assets
            (
                "wall_middle",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Walls/Walls(1)/wall(1)_mid.png",
            ), // Default wall
            (
                "wall2",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Side/wall_side_mid_left.png",
            ), // Wall with index 2
            (
                "wall3",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Side/wall_side_mid_right.png",
            ), // Wall with index 3
            (
                "wall4",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Top/wall_top_mid.png",
            ),
            (
                "wall5",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Inner_Corner/wall_inner_corner_mid_left.png",
            ),
            (
                "wall6",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Wall_Inner_Corner/wall_inner_corner_mid_rigth.png",
            ),
            // Floor asset
            (
                "floor",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Floors/Floor(1)/floor_1(1).png",
            ),
            // Decoration assets
            (
                "skull",
                "/sprites/Files/Assets/Tilesets/Tileset_1/skull.png",
            ),
            // Door asset
            (
                "door",
                "/sprites/Files/Assets/Tilesets/Tileset_1/Doors/doors_leaf_closed.png",
            ),
        ];
        
        asset_manager.load_assets(ctx, &wall_assets).expect("Failed to load wall assets");
        
        // Send registration/login command
        nc.send("register xyz 123\r\n".to_string());
        // Wait a bit for server response
        std::thread::sleep(std::time::Duration::from_millis(100));
        
        // Create the map
        let map = Map::new();
        
        // Start the player at a valid position in the map (e.g., in an open area)
        // Using grid coordinates 1,1 which should be an open space in our map
        let start_pos = Position::new(GRID_SIZE * 1.5, GRID_SIZE * 1.5);
        let players = Players::new("Player".to_string(), start_pos);
        
        Self {
            stage: Stage::PreAuth,
            nc,
            asset_manager,
            players,
            map,
        }
    }
    
    // Helper method to load assets for a specific character type
    fn load_character_assets(ctx: &mut Context, asset_manager: &mut AssetManager, character: &str) -> GameResult<()> {
        let gender = "M"; // Using male characters for now
        let character_path = format!("/sprites/Files/Assets/Heroes/{}/{}_{}",
            character, character, gender);
        
        // Load idle animations
        asset_manager.load_asset(ctx, &format!("{}_idle_down", character), 
            &format!("{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png", 
                character_path, character.to_lowercase(), gender.to_lowercase(), 
                character.to_lowercase(), gender.to_lowercase()))?;
                
        asset_manager.load_asset(ctx, &format!("{}_idle_up", character), 
            &format!("{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png", 
                character_path, character.to_lowercase(), gender.to_lowercase(), 
                character.to_lowercase(), gender.to_lowercase()))?;
                
        asset_manager.load_asset(ctx, &format!("{}_idle_right", character), 
            &format!("{}/{}_{}_idle_anim/{}_{}_idle_anim_f1.png", 
                character_path, character.to_lowercase(), gender.to_lowercase(), 
                character.to_lowercase(), gender.to_lowercase()))?;
        
        // Load idle animation frames - use idle animations for all characters
        // since not all characters have sleep animations
        for i in 1..=4 {
            let anim_path = format!("{}/{}_{}_idle_anim/{}_{}_idle_anim_f{}.png", 
                character_path, character.to_lowercase(), gender.to_lowercase(), 
                character.to_lowercase(), gender.to_lowercase(), i);
            
            asset_manager.load_asset(ctx, &format!("{}_idle_{}", character, i), &anim_path)?;
        }
        
        // Load run animations
        for i in 1..=4 {
            // Down direction
            asset_manager.load_asset(ctx, &format!("{}_run_down_{}", character, i), 
                &format!("{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png", 
                    character_path, character.to_lowercase(), gender.to_lowercase(), 
                    character.to_lowercase(), gender.to_lowercase(), i))?;
            
            // Up direction
            asset_manager.load_asset(ctx, &format!("{}_run_up_{}", character, i), 
                &format!("{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png", 
                    character_path, character.to_lowercase(), gender.to_lowercase(), 
                    character.to_lowercase(), gender.to_lowercase(), i))?;
            
            // Right direction
            asset_manager.load_asset(ctx, &format!("{}_run_right_{}", character, i), 
                &format!("{}/{}_{}_run_anim/{}_{}_run_anim_f{}.png", 
                    character_path, character.to_lowercase(), gender.to_lowercase(), 
                    character.to_lowercase(), gender.to_lowercase(), i))?;
        }
        
        // Load fallback asset
        asset_manager.load_asset(ctx, character, 
            &format!("{}/{}_{}.png", character_path, character, gender))?;
        
        Ok(())
    }

    pub fn update(&mut self, ctx: &Context) -> GameResult<()> {
        match self.stage {
            Stage::PreAuth => self.update_pre_auth(),
            Stage::InMenu => {}
            Stage::InGame => self.update_in_game(ctx),
        }

        Ok(())
    }

    fn update_pre_auth(&mut self) {
        // Handle authentication
        let line = self.nc.recv();
        match line {
            Ok(ok) => {
                println!("{}", ok);
                // Check if login was successful and transition to InGame
                if ok.contains("Logged in") || ok.contains("Registered user") {
                    log::info!("Authentication successful, entering game");
                    self.stage = Stage::InGame;
                }
            }
            Err(err) => match err {
                crate::net::NCError::NoNewData => {}
                crate::net::NCError::ConnectionError(e) => {
                    println!("Connection error: {}", e);
                }
            },
        }
    }

    fn update_in_game(&mut self, ctx: &Context) {
        // Check for server messages
        let line = self.nc.recv();
        match line {
            Ok(ok) => println!("{}", ok),
            Err(err) => match err {
                crate::net::NCError::NoNewData => {}
                crate::net::NCError::ConnectionError(e) => {
                    println!("Connection error: {}", e);
                }
            },
        }

        // Handle movement input
        let movement = input::handle_input(ctx);
        
        // Handle key press events
        let key_press = handle_key_press(ctx);
        if key_press.switch_character {
            self.players.switch_character();
        }
        
        // Update player position
        let delta_time = ctx.time.delta().as_secs_f32();
        self.players.update(&movement, &self.map, GRID_SIZE, delta_time);
        
        // Send movement to server
        input::send_movement_to_server(&mut self.nc, &movement);
        
        // Check for door transitions
        let player_center_x = self.players.self_player.pos.x + input::PLAYER_SIZE / 2.0;
        let player_center_y = self.players.self_player.pos.y + input::PLAYER_SIZE / 2.0;

        if let Some((door_x, door_y, direction)) =
            self.map
                .check_door_transition(player_center_x, player_center_y, GRID_SIZE)
        {
            // Calculate base position at the door
            let base_x = door_x as f32 * GRID_SIZE;
            let base_y = door_y as f32 * GRID_SIZE;

            // Offset the player from the door based on the direction
            // This prevents the player from immediately triggering the door again
            use input::Direction::*;
            let (a, b, c) = match direction {
                Up => (base_x, base_y - GRID_SIZE, Up),
                Down => (base_x, base_y + GRID_SIZE, Down),
                Left => (base_x - GRID_SIZE, base_y, Left),
                Right => (base_x + GRID_SIZE, base_y, Right),
            };
            self.players.self_player.pos.x = a;
            self.players.self_player.pos.y = b;
            self.players.self_player.direction = c;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        use Stage::*;
        match self.stage {
            PreAuth => self.draw_pre_auth(ctx, &mut canvas),
            InMenu => {}
            InGame => self.draw_in_game(ctx, &mut canvas),
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
        let player_center_x = self.players.self_player.pos.x + input::PLAYER_SIZE / 2.0;
        let player_center_y = self.players.self_player.pos.y + input::PLAYER_SIZE / 2.0;

        let camera_x = player_center_x - zoomed_width / 2.0;
        let camera_y = player_center_y - zoomed_height / 2.0;

        // Set the camera view
        canvas.set_screen_coordinates(Rect::new(camera_x, camera_y, zoomed_width, zoomed_height));

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
                .dest([camera_x + 10.0, camera_y + 10.0])
                .color(Color::WHITE),
        );
    }
}
