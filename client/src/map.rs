use crate::assets::AssetManager;
use ggez::{Context, GameResult, graphics};
use protocol::Facing;

// Wall types for different wall appearances
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    Wall,
    Wall2, // New wall type for wall with index 2
    Wall3, // New wall type for wall with index 3
    Wall4, // Bottom wall
    Wall5, //Top Left
    Wall6, //Top Right
    Skull, // Skull decoration on floor
    Door,  // Door to transition between rooms
}

// Define a room with its own grid layout
pub struct Room {
    grid: Vec<Vec<TileType>>,
    width: usize,
    height: usize,
    // Store decoration positions separately to draw them on top of floor tiles
    decorations: Vec<(usize, usize, TileType)>,
}

// Define the map as a collection of rooms with doors connecting them
pub struct Map {
    rooms: Vec<Room>,
    pub current_room: usize,
    // Store door positions and their destination room index
    doors: Vec<(usize, usize, usize)>, // (x, y, destination_room_index)
}

impl Room {
    pub fn new(layout: Vec<Vec<u8>>) -> Self {
        let height = layout.len();
        let width = if height > 0 { layout[0].len() } else { 0 };

        // Convert the basic grid to a grid with proper wall types
        let mut grid = vec![vec![TileType::Empty; width]; height];

        for y in 0..height {
            for x in 0..width {
                grid[y][x] = match layout[y][x] {
                    0 => TileType::Empty,
                    1 => TileType::Wall,
                    2 => TileType::Wall2,
                    3 => TileType::Wall3,
                    4 => TileType::Wall4,
                    5 => TileType::Wall5,
                    6 => TileType::Wall6,
                    7 => TileType::Door,
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
}

impl Map {
    pub fn new() -> Self {
        // First room layout
        let room1_layout = vec![
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
            vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 10)
            vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
        ];

        // Second room layout
        let room2_layout = vec![
            vec![2, 5, 1, 1, 1, 1, 1, 1, 1, 1, 6, 3],
            vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 1)
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

        // Create rooms
        let room1 = Room::new(room1_layout);
        let room2 = Room::new(room2_layout);

        // Define doors and their destinations
        // Format: (x, y, destination_room_index)
        let doors = vec![
            (7, 10, 1), // Door in room 0 (first room) at position (7, 10) leads to room 1
            (7, 1, 0),  // Door in room 1 (second room) at position (7, 1) leads to room 0
        ];

        Self {
            rooms: vec![room1, room2],
            current_room: 0,
            doors,
        }
    }

    pub fn draw(
        &self,
        _ctx: &Context,
        canvas: &mut graphics::Canvas,
        asset_manager: &AssetManager,
        grid_size: f32,
    ) -> GameResult<()> {
        let room = &self.rooms[self.current_room];

        // First draw floor tiles for all cells
        if let Some(floor_asset) = asset_manager.get_asset("floor") {
            for y in 0..room.height {
                for x in 0..room.width {
                    if room.grid[y][x] == TileType::Empty || room.grid[y][x] == TileType::Door {
                        // Draw floor at this position (doors have floor underneath)
                        let dest = [x as f32 * grid_size, y as f32 * grid_size];
                        canvas.draw(&floor_asset.img, graphics::DrawParam::default().dest(dest));
                    }
                }
            }
        }

        // Then draw wall tiles and doors on top
        for y in 0..room.height {
            for x in 0..room.width {
                match room.grid[y][x] {
                    TileType::Empty => {} // Skip empty tiles
                    TileType::Wall => {
                        // Use wall_middle for regular walls
                        if let Some(wall_asset) = asset_manager.get_asset("wall_middle") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&wall_asset.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Wall2 => {
                        // Use wall2 for the second wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall2") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&wall_asset.img, graphics::DrawParam::default().dest(dest));
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas
                                .draw(&default_wall.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Wall3 => {
                        // Use wall3 for the third wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall3") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&wall_asset.img, graphics::DrawParam::default().dest(dest));
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas
                                .draw(&default_wall.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Wall4 => {
                        // Use wall4 for the bottom wall type
                        if let Some(wall_asset) = asset_manager.get_asset("wall4") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&wall_asset.img, graphics::DrawParam::default().dest(dest));
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas
                                .draw(&default_wall.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Wall5 => {
                        // Use wall5 for the top left corner
                        if let Some(wall_asset) = asset_manager.get_asset("wall5") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&wall_asset.img, graphics::DrawParam::default().dest(dest));
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas
                                .draw(&default_wall.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Wall6 => {
                        // Use wall6 for the top right corner
                        if let Some(wall_asset) = asset_manager.get_asset("wall6") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&wall_asset.img, graphics::DrawParam::default().dest(dest));
                        } else if let Some(default_wall) = asset_manager.get_asset("wall_middle") {
                            // Fallback to default wall if specific asset not found
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas
                                .draw(&default_wall.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Door => {
                        // Draw the door
                        if let Some(door_asset) = asset_manager.get_asset("door") {
                            let dest = [x as f32 * grid_size, y as f32 * grid_size];
                            canvas.draw(&door_asset.img, graphics::DrawParam::default().dest(dest));
                        }
                    }
                    TileType::Skull => {} // Skulls are drawn separately
                }
            }
        }

        // Draw decorations on top of floor tiles
        for (x, y, tile_type) in &room.decorations {
            match tile_type {
                TileType::Skull => {
                    if let Some(skull_asset) = asset_manager.get_asset("skull") {
                        let dest = [*x as f32 * grid_size, *y as f32 * grid_size];
                        canvas.draw(&skull_asset.img, graphics::DrawParam::default().dest(dest));
                    }
                }
                _ => {} // Skip other decoration types for now
            }
        }

        Ok(())
    }

    // Check if a position is valid (not a wall)
    pub fn is_valid_position(&self, x: f32, y: f32, grid_size: f32) -> bool {
        let room = &self.rooms[self.current_room];

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
            if grid_x >= room.width || grid_y >= room.height {
                return false;
            }

            // If this corner is in any type of wall, position is invalid
            match room.grid[grid_y][grid_x] {
                TileType::Empty | TileType::Skull | TileType::Door => {} // Empty space, decorations, and doors are valid to walk on
                TileType::Wall
                | TileType::Wall2
                | TileType::Wall3
                | TileType::Wall4
                | TileType::Wall5
                | TileType::Wall6 => return false, // Any wall type is invalid
            }
        }

        // All corners are in valid positions
        true
    }

    // Check if player is on a door tile and handle room transition
    pub fn check_door_transition(
        &mut self,
        x: f32,
        y: f32,
        grid_size: f32,
    ) -> Option<(usize, usize, Facing)> {
        // Calculate the grid position of the player's center
        let center_x = (x / grid_size) as usize;
        let center_y = (y / grid_size) as usize;

        // Check if the player is standing on a door
        for (door_x, door_y, dest_room) in &self.doors {
            if *door_x == center_x && *door_y == center_y && self.current_room != *dest_room {
                // Transition to the destination room
                let prev_room = self.current_room;
                self.current_room = *dest_room;

                // Find the corresponding door in the destination room
                for (other_door_x, other_door_y, other_dest_room) in &self.doors {
                    if *other_dest_room == prev_room && *dest_room == self.current_room {
                        // Determine the direction to offset the player from the door
                        // This prevents the player from immediately triggering the door again
                        use protocol::Facing::*;
                        let direction = if *other_door_y == 1 {
                            // Door is at the top of the room, move player down
                            South
                        } else if *other_door_y == self.rooms[self.current_room].height - 2 {
                            // Door is at the bottom of the room, move player up
                            North
                        } else if *other_door_x == 1 {
                            // Door is at the left of the room, move player right
                            East
                        } else if *other_door_x == self.rooms[self.current_room].width - 2 {
                            // Door is at the right of the room, move player left
                            West
                        } else {
                            // Default direction if door position is ambiguous
                            South
                        };

                        // Return the position of the door in the new room and the direction to offset
                        return Some((*other_door_x, *other_door_y, direction));
                    }
                }
            }
        }

        None
    }
}
