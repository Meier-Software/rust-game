# Map Editor for Rust Game

A Python-based map editor for creating and editing maps for the Rust game.

## Installation

1. Make sure you have Python 3.6+ installed
2. Install the required dependencies:
   ```
   pip install -r requirements.txt
   ```

## Usage

### Basic Usage

Run the map editor:
```
python map_editor.py
```

Open an existing map:
```
python map_editor.py path/to/map.json
```

### Controls

- **Left Click**: Place the selected tile
- **Right Click + Drag**: Move the camera
- **Ctrl+S**: Save the map
- **G**: Toggle grid
- **H**: Toggle help
- **1-9**: Select tile type
- **Esc**: Quit

### Tile Types

- **1**: Empty (floor)
- **2**: Wall
- **3**: Wall2 (Left wall)
- **4**: Wall3 (Right wall)
- **5**: Wall4 (Bottom wall)
- **6**: Wall5 (Top Left corner)
- **7**: Wall6 (Top Right corner)
- **8**: Skull (decoration)
- **9**: Door (transition between rooms)

### Creating Rooms

1. Click the "Add Room" button to create a new room
2. Use the room buttons to switch between rooms
3. Place tiles to design each room

### Creating Doors

1. Select the Door tile type (9)
2. Click on the grid where you want to place the door
3. Enter the destination room number in the dialog
4. The door will be created with a connection to the specified room

## Map Format

The map editor saves maps in a JSON format compatible with the Rust game. The format includes:

- Rooms with grids of tiles
- Door connections between rooms
- Decorations

## Using Maps in the Game

After creating a map, you can use it in the Rust game:

```
cargo run --bin client -- --import-map path/to/map.json
``` 