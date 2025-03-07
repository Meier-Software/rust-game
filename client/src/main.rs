use std::path::PathBuf;
use std::time::Duration;

use ggez::{
    Context, GameResult, event,
    graphics::{self, Color, DrawParam, Image, Rect},
    input::keyboard::{self, KeyCode},
    mint::Point2,
};
use net::NetClient;
use protocol::Position;

const PLAYER_SIZE: f32 = 32.0;
const MOVEMENT_SPEED: f32 = 2.0;
const GRID_SIZE: f32 = 64.0;
const WORLD_SIZE: f32 = 800.0; // Smaller play area
const SPRITE_SHEET_WIDTH: f32 = 64.0; // Width of each sprite in the sheet
const SPRITE_SHEET_HEIGHT: f32 = 64.0; // Height of each sprite in the sheet
const ANIMATION_FRAME_TIME: f32 = 0.05; // Halved from 0.1 to make animation twice as fast
const CAMERA_ZOOM: f32 = 2.0; // Camera zoom factor (higher = more zoomed in)
const DIALOGUE_PADDING: f32 = 20.0;
const DIALOGUE_HEIGHT: f32 = 150.0;

mod net;
mod assets;

#[derive(PartialEq)]
pub enum Stage {
    PreAuth,
    InMenu,
    InGame,
}

// Direction enum for player animation
#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

pub struct GameState {
    stage: Stage,
    nc: NetClient,

    sp: Image,
    pos: Position,

    // Animation state
    current_frame: usize,
    frame_timer: f32,
    direction: Direction,
    is_moving: bool,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let mut nc = NetClient::new();

        // Load the player sprite with proper error handling
        let player_sprite =
            match Image::from_path(ctx, "/sprites/player/professor_walk_cycle_no_hat.png") {
                Ok(img) => img,
                Err(e) => {
                    println!("Failed to load sprite: {}", e);
                    // Try an alternative path as fallback
                    Image::from_path(ctx, "sprites/player/professor_walk_cycle_no_hat.png")
                        .expect("Failed to load player sprite")
                }
            };




        // Send registration/login command
        nc.send("register xyz 123\r\n".to_string());
        // Wait a bit for server response
        std::thread::sleep(Duration::from_millis(100));

        let pos = Position::new(100.0, 100.0); // Start at a more visible position

        Self {
            stage: Stage::PreAuth,
            nc,
            pos,
            sp: player_sprite,
            current_frame: 0,
            frame_timer: 0.0,
            direction: Direction::Down,
            is_moving: false,
        }
    }

    pub fn run_stage(&mut self, ctx: &mut Context) {
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
            }
        }
    }

    pub fn draw_stage(&mut self, ctx: &mut Context) {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        // TODO: Set camera here

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
                // Handle keyboard input for movement
                let mut dx = 0.0;
                let mut dy = 0.0;

                if keyboard::is_key_pressed(ctx, KeyCode::Up)
                    || keyboard::is_key_pressed(ctx, KeyCode::W)
                {
                    dy -= MOVEMENT_SPEED;
                    self.direction = Direction::Up;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::Down)
                    || keyboard::is_key_pressed(ctx, KeyCode::S)
                {
                    dy += MOVEMENT_SPEED;
                    self.direction = Direction::Down;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::Left)
                    || keyboard::is_key_pressed(ctx, KeyCode::A)
                {
                    dx -= MOVEMENT_SPEED;
                    self.direction = Direction::Left;
                }
                if keyboard::is_key_pressed(ctx, KeyCode::Right)
                    || keyboard::is_key_pressed(ctx, KeyCode::D)
                {
                    dx += MOVEMENT_SPEED;
                    self.direction = Direction::Right;
                }

                // Update animation state
                self.is_moving = dx != 0.0 || dy != 0.0;

                if self.is_moving {
                    // Update animation frame
                    self.frame_timer += ctx.time.delta().as_secs_f32();
                    if self.frame_timer >= ANIMATION_FRAME_TIME {
                        self.frame_timer = 0.0;
                        self.current_frame = (self.current_frame + 1) % 9; // Assuming 9 frames per direction
                    }
                }

                // If there's movement, update position and send to server
                if dx != 0.0 || dy != 0.0 {
                    // Update local position
                    self.pos.x += dx;
                    self.pos.y += dy;

                    // Ensure player stays within world bounds
                    self.pos.x = self.pos.x.max(0.0).min(WORLD_SIZE - PLAYER_SIZE);
                    self.pos.y = self.pos.y.max(0.0).min(WORLD_SIZE - PLAYER_SIZE);

                    // Send movement command to server
                    // Convert to integer deltas for the server
                    let dx_int = dx as i32;
                    let dy_int = dy as i32;
                    if dx_int != 0 || dy_int != 0 {
                        let move_cmd = format!("move {} {}\r\n", dx_int, dy_int);
                        self.nc.send(move_cmd);
                    }
                }

                let screen_width = ctx.gfx.window().inner_size().width as f32;
                let screen_height = ctx.gfx.window().inner_size().height as f32;

                let zoomed_width = screen_width / CAMERA_ZOOM;
                let zoomed_height = screen_height / CAMERA_ZOOM;

                canvas.set_screen_coordinates(Rect::new(0.0, 0.0, zoomed_width, zoomed_height));

                // Calculate the source rectangle for the current animation frame
                // Corrected direction mapping based on user's description:
                // Left is correct
                // Right is actually Down
                // Down is actually Up
                // Up is actually Right
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
                    src_x / self.sp.width() as f32,
                    src_y / self.sp.height() as f32,
                    SPRITE_SHEET_WIDTH / self.sp.width() as f32,
                    SPRITE_SHEET_HEIGHT / self.sp.height() as f32,
                );

                // Draw the player sprite at the correct position
                let draw_params = DrawParam::default()
                    .dest([self.pos.x, self.pos.y])
                    .scale([1.0, 1.0])
                    .src(src_rect);

                canvas.draw(&self.sp, draw_params);

                // Draw position info for debugging
                let pos_text =
                    graphics::Text::new(format!("Pos: ({:.1}, {:.1})", self.pos.x, self.pos.y));
                canvas.draw(
                    &pos_text,
                    DrawParam::default().dest([10.0, 10.0]).color(Color::WHITE),
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
