use ggez::{
    event,
    graphics::{self, Color, DrawMode, DrawParam, Image, Mesh, Rect},
    input::keyboard::KeyCode,
    Context, GameResult,
};
use glam::Vec2;
use std::path::PathBuf;

const PLAYER_SIZE: f32 = 32.0;
const MOVEMENT_SPEED: f32 = 5.0;
const GRID_SIZE: f32 = 64.0;
const WORLD_SIZE: f32 = 800.0; // Smaller play area
const SPRITE_SHEET_WIDTH: f32 = 64.0;  // Width of each sprite in the sheet
const SPRITE_SHEET_HEIGHT: f32 = 64.0; // Height of each sprite in the sheet
const ANIMATION_FRAME_TIME: f32 = 0.1; // Time between animation frames in seconds

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
}

#[derive(Clone)]
struct Cell {
    cell_type: CellType,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct GameState {
    player_pos: Vec2,
    player_direction: Direction,
    grid: Vec<Vec<Cell>>,
    player_sprite: Image,
    wall_sprite: Image,
    animation_frame: usize,
    animation_timer: f32,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        // Create a grid of empty cells
        let grid_size = (WORLD_SIZE / GRID_SIZE) as usize;
        let mut grid = vec![vec![Cell { cell_type: CellType::Empty }; grid_size]; grid_size];
        
        // Create a box around the edges
        for x in 0..grid_size {
            for y in 0..grid_size {
                // Create walls on the edges
                if x == 0 || x == grid_size - 1 || y == 0 || y == grid_size - 1 {
                    grid[x][y] = Cell {
                        cell_type: CellType::Wall,
                    };
                }
            }
        }

        // Load sprites
        let player_sprite = Self::load_player_sprite(ctx);
        let wall_sprite = Image::from_path(ctx, "/sprites/tiles/wall.png").expect("Failed to load wall sprite");

        Self {
            player_pos: Vec2::new(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            player_direction: Direction::Down,
            grid,
            player_sprite,
            wall_sprite,
            animation_frame: 0,
            animation_timer: 0.0,
        }
    }

    fn load_player_sprite(ctx: &mut Context) -> Image {
        // Load the professor sprite for the player
        Image::from_path(ctx, "/sprites/player/professor_walk_cycle_no_hat.png").expect("Failed to load player sprite")
    }

    fn check_collision(&self, pos: Vec2) -> bool {
        // Check all four corners of the player rectangle
        let corners = [
            Vec2::new(pos.x - PLAYER_SIZE / 2.0, pos.y - PLAYER_SIZE / 2.0), // Top-left
            Vec2::new(pos.x + PLAYER_SIZE / 2.0, pos.y - PLAYER_SIZE / 2.0), // Top-right
            Vec2::new(pos.x - PLAYER_SIZE / 2.0, pos.y + PLAYER_SIZE / 2.0), // Bottom-left
            Vec2::new(pos.x + PLAYER_SIZE / 2.0, pos.y + PLAYER_SIZE / 2.0), // Bottom-right
        ];

        for corner in corners.iter() {
            let grid_x = (corner.x / GRID_SIZE) as usize;
            let grid_y = (corner.y / GRID_SIZE) as usize;
            
            if grid_x >= self.grid.len() || grid_y >= self.grid[0].len() {
                return true; // Out of bounds
            }
            
            if self.grid[grid_x][grid_y].cell_type == CellType::Wall {
                return true;
            }
        }

        false
    }

    fn draw_cell(&self, ctx: &mut Context, x: usize, y: usize, canvas: &mut graphics::Canvas) -> GameResult {
        let cell = &self.grid[x][y];
        if cell.cell_type == CellType::Empty {
            return Ok(());
        }

        let draw_params = DrawParam::default()
            .dest([x as f32 * GRID_SIZE, y as f32 * GRID_SIZE])
            .scale([GRID_SIZE / self.wall_sprite.width() as f32, GRID_SIZE / self.wall_sprite.height() as f32]);
        
        canvas.draw(&self.wall_sprite, draw_params);
        Ok(())
    }

    fn draw_player(&self, canvas: &mut graphics::Canvas) -> GameResult {
        // Calculate the source rectangle for the current animation frame
        let sprite_width = self.player_sprite.width() as f32;
        let sprite_height = self.player_sprite.height() as f32;
        
        // Calculate the number of frames in the sprite sheet
        let frames_per_row = (sprite_width / SPRITE_SHEET_WIDTH) as usize;
        let frames_per_col = (sprite_height / SPRITE_SHEET_HEIGHT) as usize;
        
        // Calculate the current frame position
        let current_row = match self.player_direction {
            Direction::Down => 2,  // Bottom row
            Direction::Left => 1,  // Left row
            Direction::Right => 3, // Right row
            Direction::Up => 0,    // Top row
        };
        
        let frame_x = self.animation_frame % frames_per_row;
        let frame_y = current_row;
        
        // Calculate UV coordinates
        let src_x = (frame_x as f32 * SPRITE_SHEET_WIDTH) / sprite_width;
        let src_y = (frame_y as f32 * SPRITE_SHEET_HEIGHT) / sprite_height;
        let src_w = SPRITE_SHEET_WIDTH / sprite_width;
        let src_h = SPRITE_SHEET_HEIGHT / sprite_height;
        
        let src_rect = Rect::new(src_x, src_y, src_w, src_h);

        let draw_params = DrawParam::default()
            .dest([self.player_pos.x - PLAYER_SIZE / 2.0, self.player_pos.y - PLAYER_SIZE / 2.0])
            .src(src_rect)
            .scale([PLAYER_SIZE / SPRITE_SHEET_WIDTH, PLAYER_SIZE / SPRITE_SHEET_HEIGHT]);

        canvas.draw(&self.player_sprite, draw_params);
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let delta_time = ctx.time.delta().as_secs_f32();
        let mut new_pos = self.player_pos;
        let mut new_direction = self.player_direction;
        let mut is_moving = false;
        let mut movement = Vec2::ZERO;
        
        if ctx.keyboard.is_key_pressed(KeyCode::Left) {
            movement.x -= MOVEMENT_SPEED;
            new_direction = Direction::Left;
            is_moving = true;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Right) {
            movement.x += MOVEMENT_SPEED;
            new_direction = Direction::Right;
            is_moving = true;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Up) {
            movement.y -= MOVEMENT_SPEED;
            new_direction = Direction::Up;
            is_moving = true;
        }
        if ctx.keyboard.is_key_pressed(KeyCode::Down) {
            movement.y += MOVEMENT_SPEED;
            new_direction = Direction::Down;
            is_moving = true;
        }

        // Normalize diagonal movement
        if movement.x != 0.0 && movement.y != 0.0 {
            movement = movement.normalize() * MOVEMENT_SPEED;
        }

        // Apply movement
        new_pos.x = (self.player_pos.x + movement.x).max(0.0).min(WORLD_SIZE);
        new_pos.y = (self.player_pos.y + movement.y).max(0.0).min(WORLD_SIZE);

        // Update animation
        if is_moving {
            self.animation_timer += delta_time;
            if self.animation_timer >= ANIMATION_FRAME_TIME {
                self.animation_timer = 0.0;
                let frames_per_row = (self.player_sprite.width() as f32 / SPRITE_SHEET_WIDTH) as usize;
                self.animation_frame = (self.animation_frame + 1) % frames_per_row;
            }
        } else {
            self.animation_frame = 0; // Reset to first frame when not moving
        }

        // Only update position if there's no collision
        if !self.check_collision(new_pos) {
            self.player_pos = new_pos;
            self.player_direction = new_direction;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
        // Calculate camera position to center on player
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;
        let camera_pos = Vec2::new(
            self.player_pos.x - screen_width / 2.0,
            self.player_pos.y - screen_height / 2.0,
        );

        // Apply camera transform first
        canvas.set_screen_coordinates(Rect::new(
            camera_pos.x,
            camera_pos.y,
            screen_width,
            screen_height,
        ));

        // Draw cells
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.draw_cell(ctx, x, y, &mut canvas)?;
            }
        }
        
        // Draw the player sprite with animation
        self.draw_player(&mut canvas)?;
        canvas.finish(ctx)?;
        
        Ok(())
    }
}

fn main() -> GameResult {
    let resource_dir = PathBuf::from("./resources");
    let cb = ggez::ContextBuilder::new("simple_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Simple 2D Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);
    
    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx);
    
    event::run(ctx, event_loop, state)
}
