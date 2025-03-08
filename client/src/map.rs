use ggez::{Context, GameResult, graphics};
use crate::assets::AssetManager;

// Wall types for different wall appearances
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Wall2,  // New wall type for wall with index 2
    Wall3,  // New wall type for wall with index 3
    Wall4,  // Bottom wall
    Wall5, //Top Left
    Wall6, //Top Right
    Skull, // Skull decoration on floor
}

// Define the map as a 2D grid with different tile types
pub struct Map {
    grid: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
    // Store decoration positions separately to draw them on top of floor tiles
    decorations: Vec<(usize, usize, TileType)>,
}

impl Map {
    pub fn new() -> Self {
        // Start with a basic layout where:
        // 0 is empty, 1 is wall, 2 is wall2, 3 is wall3
        let basic_grid = vec![
            vec![2, 5, 1, 1, 1, 1, 1, 1, 1, 1, 6, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
            vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        ];

        let height = basic_grid.len();
        let width = if height > 0 { basic_grid[0].len() } else { 0 };

        // Convert the basic grid to a grid with proper wall types
        let mut grid = vec![vec![TileType::Empty; width]; height];
        
        for y in 0..height {
            for x in 0..width {
                grid[y][x] = match basic_grid[y][x] {
                    0 => TileType::Empty,
                    1 => TileType::Wall,
                    2 => TileType::Wall2,
                    3 => TileType::Wall3,
                    4 => TileType::Wall4,
                    5 => TileType::Wall5,
                    6 => TileType::Wall6,
                    _ => TileType::Wall, // Default to regular wall for any other value
                };
            }
        }

        // Create decorations list with a skull at position (5, 5)
        let decorations = vec![(5, 5, TileType::Skull)];

        Self {
            grid,
            width,
            height,
            decorations,
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
                match self.grid[y][x] {
                    TileType::Empty => {}, // Skip empty tiles
                    TileType::Wall => {
                        // Use wall_middle for regular walls
                        if let Some(wall_asset) = asset_manager.get_asset("wall_middle") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &wall_asset.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        }
                    },
                    TileType::Wall2 => {
                        // Use wall2 for the second wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall2") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &wall_asset.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &default_wall.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        }
                    },
                    TileType::Wall3 => {
                        // Use wall3 for the third wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall3") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &wall_asset.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &default_wall.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        }
                    },
                    TileType::Wall4 => {
                        // Use wall4 for the bottom wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall4") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &wall_asset.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &default_wall.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        }
                    },
                    TileType::Wall5 => {
                        // Use wall4 for the bottom wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall5") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &wall_asset.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &default_wall.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        }
                    },
                    TileType::Wall6 => {
                        // Use wall4 for the bottom wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall6") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &wall_asset.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(
                                &default_wall.img,
                                graphics::DrawParam::default()
                                    .dest(dest)
                            );
                        }
                    },
                    TileType::Skull => {}, // Skulls are drawn separately
                }
            }
        }

        // Draw decorations on top of floor tiles
        for (x, y, tile_type) in &self.decorations {
            match tile_type {
                TileType::Skull => {
                    if let Some(skull_asset) = asset_manager.get_asset("skull") {
                        let dest = [*x as f32 * grid_size, *y as f32 * grid_size];
                        canvas.draw(
                            &skull_asset.img,
                            graphics::DrawParam::default()
                                .dest(dest)
                        );
                    }
                },
                _ => {}, // Skip other decoration types for now
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
            
            // If this corner is in any type of wall, position is invalid
            match self.grid[grid_y][grid_x] {
                TileType::Empty | TileType::Skull => {}, // Empty space and decorations are valid to walk on
                TileType::Wall | TileType::Wall2 | TileType::Wall3 | TileType::Wall4 | TileType::Wall5 | TileType::Wall6 => return false, // Any wall type is invalid
            }
        }
        
        // All corners are in valid positions
        true
    }
}