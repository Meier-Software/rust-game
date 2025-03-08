use client::map::Map;
use std::env;

fn main() {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let output_path = if args.len() > 1 {
        &args[1]
    } else {
        "custom_map.json"
    };

    println!("Generating custom map to {}", output_path);
    
    // Create a custom map with three rooms
    
    // First room - entrance hall
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
        vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 10) to room 1
        vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
    ];

    // Second room - main hall with obstacles
    let room2_layout = vec![
        vec![2, 5, 1, 1, 1, 1, 1, 1, 1, 1, 6, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 1) to room 0
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 3],
        vec![2, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 3],
        vec![2, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 3],
        vec![2, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 10) to room 2
        vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
    ];
    
    // Third room - treasure room with inner chamber
    let room3_layout = vec![
        vec![2, 5, 1, 1, 1, 1, 1, 1, 1, 1, 6, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 1) to room 1
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 3],
        vec![2, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 3],
        vec![2, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0, 3],
        vec![2, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 3],
        vec![2, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 3],
        vec![2, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 10) to room 3
        vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
    ];
    
    // Fourth room - secret chamber
    let room4_layout = vec![
        vec![2, 5, 1, 1, 1, 1, 1, 1, 1, 1, 6, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 3], // Door at position (7, 1) to room 2
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3],
        vec![2, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 3],
    ];
    
    // Define doors and their destinations
    let doors = vec![
        (7, 10, 1), // Door in room 0 at position (7, 10) leads to room 1
        (7, 1, 0),  // Door in room 1 at position (7, 1) leads to room 0
        (7, 10, 2), // Door in room 1 at position (7, 10) leads to room 2
        (7, 1, 1),  // Door in room 2 at position (7, 1) leads to room 1
        (7, 10, 3), // Door in room 2 at position (7, 10) leads to room 3
        (7, 1, 2),  // Door in room 3 at position (7, 1) leads to room 2
    ];
    
    // Create the map
    let map = Map::from_layouts(
        vec![room1_layout, room2_layout, room3_layout, room4_layout],
        doors
    );
    
    // Save the map to JSON
    match map.to_json(output_path) {
        Ok(_) => println!("Custom map saved to {}", output_path),
        Err(e) => eprintln!("Error saving map: {}", e),
    }
} 