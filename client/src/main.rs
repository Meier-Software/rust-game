use std::path::PathBuf;
use std::time::Duration;

use ggez::{
    Context, GameResult, event,
    graphics::{self, Color, DrawParam, Rect},
};
use net::NetClient;
use protocol::Position;
use assets::AssetManager;

// Constants that remain in main.rs
const GRID_SIZE: f32 = 16.0;
const SPRITE_SHEET_WIDTH: f32 = 64.0; // Width of each sprite in the sheet
const SPRITE_SHEET_HEIGHT: f32 = 64.0; // Height of each sprite in the sheet
const ANIMATION_FRAME_TIME: f32 = 0.05; // Halved from 0.1 to make animation twice as fast
const CAMERA_ZOOM: f32 = 4.0; // Increased from 2.0 to 4.0 for a more zoomed in view
const DIALOGUE_PADDING: f32 = 20.0;
const DIALOGUE_HEIGHT: f32 = 150.0;

mod net;
mod assets;
mod input;
mod player;
mod map;

use map::Map;

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
    
    // Player state
    pos: Position,
    current_frame: usize,
    frame_timer: f32,
    direction: input::Direction,
    is_moving: bool,
    
    // Map
    map: Map,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let mut nc = NetClient::new();
        let mut asset_manager = AssetManager::new();
        
        // Load player sprite
        if let Err(e) = asset_manager.load_asset(
            ctx, 
            "player", 
            "/sprites/player/professor_walk_cycle_no_hat.png"
        ) {
            println!("Failed to load sprite: {}", e);
            // Try an alternative path as fallback
            asset_manager.load_asset(
                ctx,
                "player",
                "sprites/player/professor_walk_cycle_no_hat.png"
            ).expect("Failed to load player sprite");
        }
        
        // Load wall sprite
        if let Err(e) = asset_manager.load_asset(
            ctx,
            "wall",
            "/sprites/Files/Assets/Tilesets/Tileset_1/Walls/Walls/Walls(1)/wall(1)_mid.png"
        ) {
            println!("Failed to load wall sprite: {}", e);
            // Try an alternative path as fallback
            asset_manager.load_asset(
                ctx,
                "wall",
                "sprites/Files/Assets/Tilesets/Tileset_1/Walls/Walls/Walls(1)/wall(1)_mid.png"
            ).expect("Failed to load wall sprite");
        }
        
        // Load floor sprite
        if let Err(e) = asset_manager.load_asset(
            ctx,
            "floor",
            "/sprites/Files/Assets/Tilesets/Tileset_1/Floors/Floor(1)/floor_1(1).png"
        ) {
            println!("Failed to load floor sprite: {}", e);
            // Try an alternative path as fallback
            asset_manager.load_asset(
                ctx,
                "floor",
                "sprites/Files/Assets/Tilesets/Tileset_1/Floors/Floor(1)/floor_1(1).png"
            ).expect("Failed to load floor sprite");
        }

        // Send registration/login command
        nc.send("register xyz 123\r\n".to_string());
        // Wait a bit for server response
        std::thread::sleep(Duration::from_millis(100));

        // Create the map
        let map = Map::new();
        
        // Start the player at a valid position in the map (e.g., in an open area)
        // Using grid coordinates 1,1 which should be an open space in our map
        let pos = Position::new(GRID_SIZE * 1.5, GRID_SIZE * 1.5);

        Self {
            stage: Stage::PreAuth,
            nc,
            asset_manager,
            pos,
            current_frame: 0,
            frame_timer: 0.0,
            direction: input::Direction::Down,
            is_moving: false,
            map,
        }
    }

    pub fn run_stage(&mut self, ctx: &mut ggez::Context) {
        match self.stage {
            Stage::PreAuth => {
                // println!("Pre Auth");
                let line = self.nc.recv();
                match line {
                    Ok(ok) => {
                        println!("{}", ok);
                        // Check if login was successful and transition to InGame
                        if ok.contains("Logged in") || ok.contains("Registered user") {
                            println!("Authentication successful, entering game");
                            self.stage = Stage::InGame;
                        }
                    }
                    Err(err) => match err {
                        net::NCError::NoNewData => {}
                        net::NCError::ConnectionError(e) => {
                            println!("Connection error: {}", e);
                        }
                    },
                }
            }
            Stage::InMenu => {}
            Stage::InGame => {
                // Check for server messages
                let line = self.nc.recv();
                // println!("{:?}", line);
                match line {
                    Ok(ok) => println!("{}", ok),
                    Err(err) => match err {
                        net::NCError::NoNewData => {}
                        net::NCError::ConnectionError(e) => {
                            println!("Connection error: {}", e);
                            // Optionally transition back to PreAuth stage
                            // self.stage = Stage::PreAuth;
                        }
                    },
                }
                
                // Handle movement using the input module
                let movement = input::handle_input(ctx);
                
                // Update movement state
                self.is_moving = movement.is_moving;
                self.direction = movement.direction;
                
                // Update animation if moving
                if self.is_moving {
                    // Update animation frame
                    self.frame_timer += ctx.time.delta().as_secs_f32();
                    if self.frame_timer >= ANIMATION_FRAME_TIME {
                        self.frame_timer = 0.0;
                        self.current_frame = (self.current_frame + 1) % 9; // Assuming 9 frames per direction
                    }
                }
                
                // Update position
                input::update_position(&mut self.pos, &movement, &self.map, GRID_SIZE);
                
                // Send movement to server
                input::send_movement_to_server(&mut self.nc, &movement);
            }
        }
    }

    pub fn draw_stage(&mut self, ctx: &mut Context) {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        match self.stage {
            Stage::PreAuth => {
                // Draw login/authentication screen
                let screen_width = ctx.gfx.window().inner_size().width as f32;
                let screen_height = ctx.gfx.window().inner_size().height as f32;

                // Draw text for login screen
                let text = graphics::Text::new("Authenticating...");
                let text_pos = [screen_width / 2.0 - 50.0, screen_height / 2.0];
                canvas.draw(
                    &text,
                    DrawParam::default().dest(text_pos).color(Color::WHITE),
                );
            }
            Stage::InMenu => {}
            Stage::InGame => {
                let screen_width = ctx.gfx.window().inner_size().width as f32;
                let screen_height = ctx.gfx.window().inner_size().height as f32;

                let zoomed_width = screen_width / CAMERA_ZOOM;
                let zoomed_height = screen_height / CAMERA_ZOOM;
                
                // Center the camera on the player's center (not top-left corner)
                // Add half the player size to center on the player sprite
                let player_center_x = self.pos.x + input::PLAYER_SIZE / 2.0;
                let player_center_y = self.pos.y + input::PLAYER_SIZE / 2.0;
                
                let camera_x = player_center_x - zoomed_width / 2.0;
                let camera_y = player_center_y - zoomed_height / 2.0;
                
                // Set the camera view
                canvas.set_screen_coordinates(Rect::new(
                    camera_x, 
                    camera_y, 
                    zoomed_width, 
                    zoomed_height
                ));
                
                // Draw the map first (so it's behind the player)
                self.map.draw(ctx, &mut canvas, &self.asset_manager, GRID_SIZE).unwrap();

                // Get player sprite
                if let Some(player_asset) = self.asset_manager.get_asset("player") {
                    // Calculate the source rectangle for the current animation frame
                    let direction_offset = match self.direction {
                        input::Direction::Up => 0,
                        input::Direction::Left => 1,
                        input::Direction::Down => 2,
                        input::Direction::Right => 3,
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
                        .scale([input::PLAYER_SIZE / SPRITE_SHEET_WIDTH, input::PLAYER_SIZE / SPRITE_SHEET_HEIGHT])
                        .src(src_rect);

                    canvas.draw(&player_asset.img, draw_params);
                }

                // Draw position info for debugging - fixed to the camera view
                let pos_text =
                    graphics::Text::new(format!("Pos: ({:.1}, {:.1})", self.pos.x, self.pos.y));
                
                // Draw UI elements in screen coordinates by adding the camera position
                canvas.draw(
                    &pos_text,
                    DrawParam::default()
                        .dest([camera_x + 10.0, camera_y + 10.0])
                        .color(Color::WHITE),
                );
            }
        }

        canvas.finish(ctx).unwrap();
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    // Update once per tick.
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.run_stage(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.draw_stage(ctx);
        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = PathBuf::from("./client/assets");
    let cb = ggez::ContextBuilder::new("simple_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Simple 2D Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx);

    event::run(ctx, event_loop, state)
}
