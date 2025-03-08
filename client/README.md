# Rust Game Client

A simple 2D game client built with ggez.

## Map System

The game uses a JSON-based map system that allows for easy creation and modification of maps.

### Map Structure

Maps consist of:
- Rooms: Each room has a grid of tiles and decorations
- Doors: Connections between rooms
- Tile types: Different types of tiles (walls, floors, doors, etc.)

### Using Maps

The game supports the following map-related features:

1. **Default Map**: The game will automatically load the default map from `assets/default_map.json`.

2. **Exporting Maps**: You can export the current map to a JSON file:
   ```
   cargo run --bin client -- --export-map [output_path]
   ```

3. **Importing Maps**: You can import a custom map from a JSON file:
   ```
   cargo run --bin client -- --import-map [input_path]
   ```

4. **Creating Custom Maps**: You can use the map_generator tool to create custom maps:
   ```
   cargo run --bin map_generator [output_path]
   ```

### Map Format

The map JSON format includes:
- `rooms`: An array of rooms, each with a grid of tile types
- `current_room`: The index of the starting room
- `doors`: An array of door connections between rooms

### Tile Types

The following tile types are available:
- `Empty`: Empty space (floor)
- `Wall`: Standard wall
- `Wall2`, `Wall3`, `Wall4`, `Wall5`, `Wall6`: Different wall types
- `Skull`: Skull decoration
- `Door`: Door to transition between rooms

## Running the Game

Run the game in online mode:
```
cargo run --bin client
```

Run the game in offline mode:
```
cargo run --bin client -- --offline
```

Run the game with a custom map:
```
cargo run --bin client -- --import-map custom_map.json
``` 