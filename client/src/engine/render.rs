use ggez::Context;
use ggez::graphics::{self, Color, DrawMode, DrawParam, Image, Mesh, Rect, Canvas};
use glam::Vec2;
use protocol::{Facing, Position};
use specs::{ReadStorage, System, WriteStorage, Join, Read, Write};
use specs::World;

use super::{Engine, net::NetClient};

// Constants for player rendering
const PLAYER_SIZE: f32 = 32.0;
const SPRITE_SHEET_WIDTH: f32 = 64.0;
const SPRITE_SHEET_HEIGHT: f32 = 64.0;
const ANIMATION_FRAME_TIME: f32 = 0.05;
const CAMERA_ZOOM: f32 = 2.0; // Camera zoom factor

pub struct PlayerSprite {
    pub sprite: Image,
    pub animation_frame: usize,
    pub animation_timer: f32,
}

impl PlayerSprite {
    pub fn new(ctx: &mut Context) -> Self {
        let sprite = Image::from_path(ctx, "/sprites/player/professor_walk_cycle_no_hat.png")
            .expect("Failed to load player sprite");
        
        Self {
            sprite,
            animation_frame: 0,
            animation_timer: 0.0,
        }
    }
}

// Resource to hold the canvas for rendering
pub struct RenderCanvas {
    pub canvas: Option<Canvas>,
}

impl RenderCanvas {
    pub fn new() -> Self {
        Self { canvas: None }
    }
}

impl Engine {
    // Initialize the rendering resources
    pub fn init_rendering(&mut self, ctx: &mut Context) {
        // Create and insert the player sprite resource
        let player_sprite = PlayerSprite::new(ctx);
        self.world.insert(player_sprite);
        
        // Create and insert the render canvas resource
        let render_canvas = RenderCanvas::new();
        self.world.insert(render_canvas);
    }
    
    // Updated once per frame. FPSRate
    pub fn fps_update(&mut self, ctx: &mut Context) {
        // Create a canvas to draw on
        let canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);
        
        // Store the canvas in the resource
        {
            let mut render_canvas = self.world.write_resource::<RenderCanvas>();
            render_canvas.canvas = Some(canvas);
        }
        
        // Run the render system
        let mut render_system = RenderFrame;
        render_system.run_now(&self.world);
        
        // Finish drawing
        {
            let mut render_canvas = self.world.write_resource::<RenderCanvas>();
            if let Some(canvas) = render_canvas.canvas.take() {
                canvas.finish(ctx).expect("Failed to finish canvas");
            }
        }
    }
}

pub struct RenderFrame;

impl<'ecs_life> System<'ecs_life> for RenderFrame {
    // SystemData is what you are requesting from the world.
    type SystemData = (
        ReadStorage<'ecs_life, Position>,
        ReadStorage<'ecs_life, Facing>,
        Read<'ecs_life, PlayerSprite>,
        Write<'ecs_life, RenderCanvas>,
    );

    /// Render a frame here.
    fn run(&mut self, data: Self::SystemData) {
        let (positions, facings, player_sprite, mut render_canvas) = data;
        
        // Get the canvas from the resource
        if let Some(canvas) = &mut render_canvas.canvas {
            // Find the player position (assuming the first entity with both Position and Facing is the player)
            let mut player_pos = None;
            for (position, _) in (&positions, &facings).join() {
                player_pos = Some(Vec2::new(position.x, position.y));
                break;
            }
            
            // Apply camera transform if player position is found
            if let Some(player_position) = player_pos {
                // Get screen dimensions
                let screen_width = canvas.screen_coordinates().unwrap().w;
                let screen_height = canvas.screen_coordinates().unwrap().h;
                
                // Calculate zoomed dimensions
                let zoomed_width = screen_width / CAMERA_ZOOM;
                let zoomed_height = screen_height / CAMERA_ZOOM;
                
                // Center camera on player
                let camera_x = player_position.x - zoomed_width / 2.0;
                let camera_y = player_position.y - zoomed_height / 2.0;
                
                // Apply camera transform
                canvas.set_screen_coordinates(Rect::new(
                    camera_x,
                    camera_y,
                    zoomed_width,
                    zoomed_height,
                ));
            }
            
            // Get player position & direction.
            for (position, facing) in (&positions, &facings).join() {
                // Calculate the source rectangle for the current animation frame
                let sprite_width = player_sprite.sprite.width() as f32;
                let sprite_height = player_sprite.sprite.height() as f32;

                // Calculate the number of frames in the sprite sheet
                let frames_per_row = (sprite_width / SPRITE_SHEET_WIDTH) as usize;
                
                // Calculate the current frame position based on facing direction
                let current_row = match facing {
                    Facing::North => 0, // Up row
                    Facing::East => 3,  // Right row
                    Facing::South => 2, // Down row
                    Facing::West => 1,  // Left row
                };

                let frame_x = player_sprite.animation_frame % frames_per_row;
                let frame_y = current_row;

                // Calculate UV coordinates
                let src_x = (frame_x as f32 * SPRITE_SHEET_WIDTH) / sprite_width;
                let src_y = (frame_y as f32 * SPRITE_SHEET_HEIGHT) / sprite_height;
                let src_w = SPRITE_SHEET_WIDTH / sprite_width;
                let src_h = SPRITE_SHEET_HEIGHT / sprite_height;

                let src_rect = Rect::new(src_x, src_y, src_w, src_h);

                // Create draw parameters
                let draw_params = DrawParam::default()
                    .dest([
                        position.x - PLAYER_SIZE / 2.0,
                        position.y - PLAYER_SIZE / 2.0,
                    ])
                    .src(src_rect)
                    .scale([
                        PLAYER_SIZE / SPRITE_SHEET_WIDTH,
                        PLAYER_SIZE / SPRITE_SHEET_HEIGHT,
                    ]);

                // Draw the player sprite
                canvas.draw(&player_sprite.sprite, draw_params);
            }
        }
    }
}
