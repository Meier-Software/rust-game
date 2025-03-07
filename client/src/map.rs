use ggez::{Context, GameResult, graphics};
use crate::assets::AssetManager;

// Define the map as a 2D grid where 1 represents a wall and 0 represents an empty space
pub struct Map {
    grid: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new() -> Self {
        // Example map layout - this can be expanded or loaded from a file
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 1],
            vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            vec![1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        Self {
            grid,
            width,
            height,
        }
    }

    pub fn draw(&self, _ctx: &mut Context, canvas: &mut graphics::Canvas, asset_manager: &AssetManager, grid_size: f32) -> GameResult<()> {
        // Get the wall asset
        if let Some(wall_asset) = asset_manager.get_asset("wall") {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid[y][x] == 1 {
                        // Draw wall at this position
                        let dest = [x as f32 * grid_size, y as f32 * grid_size];
                        canvas.draw(
                            &wall_asset.img,
                            graphics::DrawParam::default()
                                .dest(dest)
                        );
                    }
                }
            }
        }
        Ok(())
    }

    // Check if a position is valid (not a wall)
    pub fn is_valid_position(&self, x: f32, y: f32, grid_size: f32) -> bool {
        // Calculate the player's hitbox corners (assuming player is centered at x,y)
        let player_half_size = crate::input::PLAYER_SIZE / 2.0;
        
        // Check all four corners of the player's hitbox
        let corners = [
            (x - player_half_size, y - player_half_size), // Top-left
            (x + player_half_size, y - player_half_size), // Top-right
            (x - player_half_size, y + player_half_size), // Bottom-left
            (x + player_half_size, y + player_half_size), // Bottom-right
        ];
        
        // Check if any corner is in a wall
        for (corner_x, corner_y) in corners {
            let grid_x = (corner_x / grid_size) as usize;
            let grid_y = (corner_y / grid_size) as usize;
            
            // Check bounds
            if grid_x >= self.width || grid_y >= self.height {
                return false;
            }
            
            // If this corner is in a wall, position is invalid
            if self.grid[grid_y][grid_x] == 1 {
                return false;
            }
        }
        
        // All corners are in valid positions
        true
    }
}