use ggez::{
    event,
    graphics::{self, Color, DrawMode, DrawParam, Image, Mesh, Rect, Text},
    input::keyboard::KeyCode,
    Context, GameResult,
};
use glam::Vec2;
use std::path::PathBuf;

const PLAYER_SIZE: f32 = 32.0;
const MOVEMENT_SPEED: f32 = 2.0;
const GRID_SIZE: f32 = 64.0;
const WORLD_SIZE: f32 = 800.0; // Smaller play area
const SPRITE_SHEET_WIDTH: f32 = 64.0;  // Width of each sprite in the sheet

// Define multiple map layouts (0 = empty, 1 = wall, 3 = door)
const MAP_1: &[&[i32]] = &[
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1],
    &[1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const MAP_2: &[&[i32]] = &[
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    &[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 1, 0, 1, 1, 1, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    &[3, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1],
    &[1, 0, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1],
    &[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    &[1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
    &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
];

const SPRITE_SHEET_HEIGHT: f32 = 64.0; // Height of each sprite in the sheet
const ANIMATION_FRAME_TIME: f32 = 0.05; // Halved from 0.1 to make animation twice as fast
const CAMERA_ZOOM: f32 = 2.0; // Camera zoom factor (higher = more zoomed in)
const DIALOGUE_PADDING: f32 = 20.0;
const DIALOGUE_HEIGHT: f32 = 150.0;

#[derive(Clone, Copy, PartialEq)]
enum CellType {
    Empty,
    Wall,
    Door,
    Coin,
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
    show_dialogue: bool,
    dialogue_text: String,
    current_map: usize, // Track which map we're currently on
    door_cooldown: f32, // Cooldown timer for door transitions
    score: u32,  // Add score tracking
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let grid = Self::create_grid_from_map(MAP_1);
        
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
            show_dialogue: false,
            dialogue_text: String::from("It's dark, I should get out of here."),
            current_map: 1,
            door_cooldown: 0.0,
            score: 0,  // Initialize score
        }
    }

    fn create_grid_from_map(map: &[&[i32]]) -> Vec<Vec<Cell>> {
        let grid_size = map.len();
        let mut grid = vec![vec![Cell { cell_type: CellType::Empty }; grid_size]; grid_size];
        
        for (y, row) in map.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                grid[x][y] = Cell {
                    cell_type: match tile {
                        1 => CellType::Wall,
                        3 => CellType::Door,
                        _ => CellType::Empty,
                    }
                };
            }
        }
        grid
    }

    fn switch_map(&mut self) {
        // Get the current door position
        let grid_x = (self.player_pos.x / GRID_SIZE) as usize;
        
        // Determine if we're entering from left or right side
        let entering_from_left = grid_x <= 1;
        
        if self.current_map == 1 {
            self.grid = Self::create_grid_from_map(MAP_2);
            self.current_map = 2;
            
            if entering_from_left {
                // Place player just to the right of the right door
                self.player_pos = Vec2::new(10.0 * GRID_SIZE, 5.0 * GRID_SIZE + GRID_SIZE/2.0);
            } else {
                // Place player just to the left of the left door
                self.player_pos = Vec2::new(2.0 * GRID_SIZE, 5.0 * GRID_SIZE + GRID_SIZE/2.0);
            }
        } else {
            self.grid = Self::create_grid_from_map(MAP_1);
            self.current_map = 1;
            
            if entering_from_left {
                // Place player just to the right of the right door
                self.player_pos = Vec2::new(10.0 * GRID_SIZE, 5.0 * GRID_SIZE + GRID_SIZE/2.0);
            } else {
                // Place player just to the left of the left door
                self.player_pos = Vec2::new(2.0 * GRID_SIZE, 5.0 * GRID_SIZE + GRID_SIZE/2.0);
            }
        }
        // Reset door cooldown
        self.door_cooldown = 0.5; // Half a second cooldown
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

    fn check_door(&self, pos: Vec2) -> bool {
        // Get the center position of the player
        let grid_x = (pos.x / GRID_SIZE) as usize;
        let grid_y = (pos.y / GRID_SIZE) as usize;
        
        // Check if we're within bounds and on a door
        if grid_x < self.grid.len() && grid_y < self.grid[0].len() {
            return self.grid[grid_x][grid_y].cell_type == CellType::Door;
        }
        false
    }

    fn draw_cell(&self, ctx: &mut Context, x: usize, y: usize, canvas: &mut graphics::Canvas) -> GameResult {
        let cell = &self.grid[x][y];
        match cell.cell_type {
            CellType::Empty => Ok(()),
            CellType::Wall => {
                let draw_params = DrawParam::default()
                    .dest([x as f32 * GRID_SIZE, y as f32 * GRID_SIZE])
                    .scale([GRID_SIZE / self.wall_sprite.width() as f32, GRID_SIZE / self.wall_sprite.height() as f32]);
                canvas.draw(&self.wall_sprite, draw_params);
                Ok(())
            },
            CellType::Door => {
                // Draw a blue rectangle for the door
                let door_rect = Rect::new(
                    x as f32 * GRID_SIZE,
                    y as f32 * GRID_SIZE,
                    GRID_SIZE,
                    GRID_SIZE,
                );
                let door_mesh = Mesh::new_rectangle(
                    ctx,
                    DrawMode::fill(),
                    door_rect,
                    Color::from_rgb(0, 0, 255), // Blue color
                )?;
                canvas.draw(&door_mesh, DrawParam::default());
                Ok(())
            }
        }
    }

    fn load_player_sprite(ctx: &mut Context) -> Image {
        // Load the professor sprite for the player
        Image::from_path(ctx, "/sprites/player/professor_walk_cycle_no_hat.png").expect("Failed to load player sprite")
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

    fn draw_dialogue(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        if !self.show_dialogue {
            return Ok(());
        }

        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;

        // Create dialogue box background
        let dialogue_rect = Rect::new(
            DIALOGUE_PADDING,
            screen_height - DIALOGUE_HEIGHT - DIALOGUE_PADDING,
            screen_width - (DIALOGUE_PADDING * 2.0),
            DIALOGUE_HEIGHT,
        );

        let dialogue_mesh = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            dialogue_rect,
            Color::from_rgba(0, 0, 0, 200),
        )?;

        // Create text
        let mut text = Text::new(self.dialogue_text.as_str());
        text.set_scale(24.0);
        text.set_bounds([dialogue_rect.w - DIALOGUE_PADDING * 2.0, dialogue_rect.h]);

        // Draw dialogue box and text
        canvas.draw(&dialogue_mesh, DrawParam::default());
        canvas.draw(&text, DrawParam::default().dest([
            dialogue_rect.x + DIALOGUE_PADDING,
            dialogue_rect.y + DIALOGUE_PADDING,
        ]));

        Ok(())
    }

    // Add function to check for coin collection
    fn check_coin(&mut self, pos: Vec2) -> bool {
        let grid_x = (pos.x / GRID_SIZE) as usize;
        let grid_y = (pos.y / GRID_SIZE) as usize;
        
        if grid_x < self.grid.len() && grid_y < self.grid[0].len() {
            if self.grid[grid_x][grid_y].cell_type == CellType::Coin {
                // Collect the coin
                self.grid[grid_x][grid_y].cell_type = CellType::Empty;
                self.score += 10;  // Add 10 points per coin
                return true;
            }
        }
        false
    }

    // Add function to draw score
    fn draw_score(&self, ctx: &mut Context, canvas: &mut graphics::Canvas) -> GameResult {
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        
        let score_text = Text::new(format!("Score: {}", self.score));
        canvas.draw(&score_text, DrawParam::default().dest([10.0, 10.0]));
        Ok(())
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update door cooldown
        if self.door_cooldown > 0.0 {
            self.door_cooldown -= ctx.time.delta().as_secs_f32();
        }

        // Handle Enter key for dialogue
        if ctx.keyboard.is_key_just_pressed(KeyCode::Return) {
            self.show_dialogue = !self.show_dialogue;
        }

        // Only process movement if dialogue is not showing
        if !self.show_dialogue {
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

            // Check for door before updating position (only if cooldown is expired)
            if self.door_cooldown <= 0.0 && self.check_door(new_pos) {
                self.switch_map();
                return Ok(());
            }

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

            // Check for coin collection at new position
            self.check_coin(new_pos);

            // Only update position if there's no collision
            if !self.check_collision(new_pos) {
                self.player_pos = new_pos;
                self.player_direction = new_direction;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
        // Calculate camera position to center on player with zoom
        let screen_width = ctx.gfx.window().inner_size().width as f32;
        let screen_height = ctx.gfx.window().inner_size().height as f32;
        let zoomed_width = screen_width / CAMERA_ZOOM;
        let zoomed_height = screen_height / CAMERA_ZOOM;
        let camera_pos = Vec2::new(
            self.player_pos.x - zoomed_width / 2.0,
            self.player_pos.y - zoomed_height / 2.0,
        );

        // Apply camera transform with zoom
        canvas.set_screen_coordinates(Rect::new(
            camera_pos.x,
            camera_pos.y,
            zoomed_width,
            zoomed_height,
        ));

        // Draw cells
        for x in 0..self.grid.len() {
            for y in 0..self.grid[0].len() {
                self.draw_cell(ctx, x, y, &mut canvas)?;
            }
        }
        
        // Draw the player sprite with animation
        self.draw_player(&mut canvas)?;

        // Reset to screen coordinates for UI elements
        canvas.set_screen_coordinates(Rect::new(
            0.0,
            0.0,
            screen_width,
            screen_height,
        ));

        // Draw dialogue box on top of everything else
        self.draw_dialogue(ctx, &mut canvas)?;

        // Draw score on top of everything
        self.draw_score(ctx, &mut canvas)?;
        
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
