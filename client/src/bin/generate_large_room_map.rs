use client::map::Map;
use std::path::Path;

fn main() {
    let output_path = "client/assets/large_room_map.json";

    // Ensure the directory exists
    if let Some(parent) = Path::new(output_path).parent() {
        if !parent.exists() {
            println!("Creating directory: {:?}", parent);
            std::fs::create_dir_all(parent).expect("Failed to create directory");
        }
    }

    println!("Generating large room map to {}", output_path);

    // Create a large room layout (30x30)
    let mut large_room = Vec::new();

    // Top wall row
    let mut top_row = Vec::new();
    top_row.push(2); // Top-left corner
    for _ in 0..28 {
        top_row.push(1); // Top wall
    }
    top_row.push(3); // Top-right corner
    large_room.push(top_row);

    // Middle rows
    for _ in 0..28 {
        let mut row = Vec::new();
        row.push(2); // Left wall
        for _ in 0..28 {
            row.push(0); // Empty space
        }
        row.push(3); // Right wall
        large_room.push(row);
    }

    // Bottom wall row
    let mut bottom_row = Vec::new();
    bottom_row.push(2); // Bottom-left corner
    for _ in 0..28 {
        bottom_row.push(1); // Bottom wall
    }
    bottom_row.push(3); // Bottom-right corner
    large_room.push(bottom_row);

    // Add some decorations (skulls) in the room
    let mut decorated_room = large_room.clone();

    // Add some skulls in specific positions
    decorated_room[5][5] = 7; // Skull at (5,5)
    decorated_room[5][25] = 7; // Skull at (5,25)
    decorated_room[25][5] = 7; // Skull at (25,5)
    decorated_room[25][25] = 7; // Skull at (25,25)
    decorated_room[15][15] = 7; // Skull at center (15,15)

    // Create the map with a single room and no doors
    let map = Map::from_layouts(vec![decorated_room], vec![]);

    // Save the map to JSON
    match map.to_json(output_path) {
        Ok(_) => println!("Large room map saved to {}", output_path),
        Err(e) => eprintln!("Error saving map: {}", e),
    }
}
