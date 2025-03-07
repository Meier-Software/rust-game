use ggez::{Context, GameResult, graphics};
use crate::assets::AssetManager;

// Wall types for different wall appearances
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    WallMiddle,
    WallLeft,
    WallRight,
}

// Define the map as a 2D grid with different tile types
pub struct Map {
    grid: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
}

impl Map {
    pub fn new() -> Self {
        // Start with a basic layout where 0 is empty and 1+ are different wall types
        // We'll convert this to proper wall types
        let basic_grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 1, 1, 1, 1, 0, 0, 0, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1],
            vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
        ];

        let height = basic_grid.len();
        let width = if height > 0 { basic_grid[0].len() } else { 0 };

        // Convert the basic grid to a grid with proper wall types
        let mut grid = vec![vec![TileType::Empty; width]; height];
        
        for y in 0..height {
            for x in 0..width {
                if basic_grid[y][x] == 0 {
                    grid[y][x] = TileType::Empty;
                } else {
                    // Determine wall type based on surrounding walls
                    grid[y][x] = Self::determine_wall_type(&basic_grid, x, y, width, height);
                }
            }
        }

        Self {
            grid,
            width,
            height,
        }
    }

    // Helper function to determine the appropriate wall type based on surrounding walls
    fn determine_wall_type(grid: &Vec<Vec<u8>>, x: usize, y: usize, width: usize, height: usize) -> TileType {
        // Check if we're at an edge
        let is_left = x == 0;
        let is_right = x == width - 1;

        // Check surrounding cells (if they exist)
        let has_left = !is_left && grid[y][x-1] == 1;
        let has_right = !is_right && grid[y][x+1] == 1;

        // More explicit logic for determining wall types
        // Left edge of a wall section (has wall to the right but not to the left)
        if !has_left && has_right {
            TileType::WallLeft
        } 
        // Right edge of a wall section (has wall to the left but not to the right)
        else if has_left && !has_right {
            TileType::WallRight
        } 
        // Middle of a wall section or standalone wall
        else {
            TileType::WallMiddle
        }
    }

    pub fn draw(&self, ctx: &Context, canvas: &mut graphics::Canvas, asset_manager: &AssetManager, grid_size: f32) -> GameResult<()> {
        // First draw floor tiles for all cells
        if let Some(floor_asset) = asset_manager.get_asset("floor") {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid[y][x] == TileType::Empty {
                        // Draw floor at this position
                        let dest = [x as f32 * grid_size, y as f32 * grid_size];
                        canvas.draw(
                            &floor_asset.img,
                            graphics::DrawParam::default()
                                .dest(dest)
                        );
                    }
                }
            }
        }
        
        // Then draw wall tiles on top
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] != TileType::Empty {
                    // Determine which wall asset to use based on the wall type
                    let asset_name = match self.grid[y][x] {
                        TileType::WallMiddle => "wall_middle",
                        TileType::WallLeft => "wall_left",
                        TileType::WallRight => "wall_right",
                        _ => "wall_middle", // Fallback
                    };
                    
                    // Draw the appropriate wall asset
                    if let Some(wall_asset) = asset_manager.get_asset(asset_name) {
                        let dest = [x as f32 * grid_size, y as f32 * grid_size];
                        canvas.draw(
                            &wall_asset.img,
                            graphics::DrawParam::default()
                                .dest(dest)
                        );
                    } else if let Some(default_wall) = asset_manager.get_asset("wall") {
                        // Fallback to default wall if specific asset not found
                        let dest = [x as f32 * grid_size, y as f32 * grid_size];
                        canvas.draw(
                            &default_wall.img,
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
        // Calculate the player's hitbox corners with a slightly smaller hitbox for better collision
        let player_half_size = crate::input::PLAYER_SIZE / 2.5; // Reduced from 2.0 to 2.5 for tighter collision
        
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
            if self.grid[grid_y][grid_x] != TileType::Empty {
                return false;
            }
        }
        
        // All corners are in valid positions
        true
    }
}