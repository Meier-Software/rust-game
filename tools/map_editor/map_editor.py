#!/usr/bin/env python3
"""
Map Editor for Rust Game
------------------------
A simple map editor for creating and editing maps for the Rust game.
"""

import os
import sys
import json
import pygame
import argparse
from enum import Enum
from typing import List, Dict, Tuple, Optional, Any

# Initialize pygame
pygame.init()

# Constants
GRID_SIZE = 40  # Size of each grid cell in pixels
WINDOW_WIDTH = 1280
WINDOW_HEIGHT = 720
FPS = 60

# Colors
BLACK = (0, 0, 0)
WHITE = (255, 255, 255)
GRAY = (128, 128, 128)
LIGHT_GRAY = (200, 200, 200)
RED = (255, 0, 0)
GREEN = (0, 255, 0)
BLUE = (0, 0, 255)
YELLOW = (255, 255, 0)
BROWN = (139, 69, 19)

# Define tile types to match the Rust game
class TileType(Enum):
    Empty = 0
    Wall = 1
    Wall2 = 2  # Left wall
    Wall3 = 3  # Right wall
    Wall4 = 4  # Bottom wall
    Wall5 = 5  # Top Left corner
    Wall6 = 6  # Top Right corner
    Skull = 7  # Skull decoration
    Door = 8   # Door to transition between rooms

# Tile colors for rendering
TILE_COLORS = {
    TileType.Empty: WHITE,
    TileType.Wall: GRAY,
    TileType.Wall2: LIGHT_GRAY,
    TileType.Wall3: LIGHT_GRAY,
    TileType.Wall4: LIGHT_GRAY,
    TileType.Wall5: LIGHT_GRAY,
    TileType.Wall6: LIGHT_GRAY,
    TileType.Skull: RED,
    TileType.Door: GREEN,
}

class Room:
    def __init__(self, width: int = 12, height: int = 12):
        self.width = width
        self.height = height
        self.grid = [[TileType.Empty for _ in range(width)] for _ in range(height)]
        self.decorations = []  # List of (x, y, TileType) for decorations
        
        # Initialize with walls around the edges
        for x in range(width):
            self.grid[0][x] = TileType.Wall  # Top wall
            self.grid[height-1][x] = TileType.Wall  # Bottom wall
        
        for y in range(height):
            self.grid[y][0] = TileType.Wall  # Left wall
            self.grid[y][width-1] = TileType.Wall  # Right wall
            
        # Set corners
        self.grid[0][0] = TileType.Wall5  # Top-left corner
        self.grid[0][width-1] = TileType.Wall6  # Top-right corner
        
        # Set side walls
        for y in range(1, height-1):
            self.grid[y][0] = TileType.Wall2  # Left wall
            self.grid[y][width-1] = TileType.Wall3  # Right wall

    def set_tile(self, x: int, y: int, tile_type: TileType):
        if 0 <= x < self.width and 0 <= y < self.height:
            self.grid[y][x] = tile_type

    def get_tile(self, x: int, y: int) -> TileType:
        if 0 <= x < self.width and 0 <= y < self.height:
            return self.grid[y][x]
        return TileType.Empty

    def add_decoration(self, x: int, y: int, tile_type: TileType):
        self.decorations.append((x, y, tile_type))

    def remove_decoration(self, x: int, y: int):
        self.decorations = [d for d in self.decorations if d[0] != x or d[1] != y]

    def to_dict(self) -> Dict:
        return {
            "grid": [[tile.name for tile in row] for row in self.grid],
            "width": self.width,
            "height": self.height,
            "decorations": [(x, y, tile_type.name) for x, y, tile_type in self.decorations]
        }

    @classmethod
    def from_dict(cls, data: Dict) -> 'Room':
        room = cls(data["width"], data["height"])
        
        # Load grid
        for y, row in enumerate(data["grid"]):
            for x, tile_name in enumerate(row):
                room.grid[y][x] = TileType[tile_name]
        
        # Load decorations
        room.decorations = [(x, y, TileType[tile_type]) for x, y, tile_type in data["decorations"]]
        
        return room

class Map:
    def __init__(self):
        self.rooms = [Room()]
        self.current_room = 0
        self.doors = []  # List of (x, y, source_room, destination_room)

    def add_room(self, width: int = 12, height: int = 12) -> int:
        self.rooms.append(Room(width, height))
        return len(self.rooms) - 1

    def add_door(self, x: int, y: int, destination_room: int):
        # Set the tile to a door
        self.rooms[self.current_room].set_tile(x, y, TileType.Door)
        
        # Add the door connection
        self.doors.append((x, y, self.current_room, destination_room))

    def remove_door(self, x: int, y: int):
        # Remove the door connection
        self.doors = [d for d in self.doors if not (d[0] == x and d[1] == y and d[2] == self.current_room)]
        
        # Set the tile back to empty
        self.rooms[self.current_room].set_tile(x, y, TileType.Empty)

    def get_door_destination(self, x: int, y: int) -> Optional[int]:
        for door_x, door_y, source_room, dest_room in self.doors:
            if door_x == x and door_y == y and source_room == self.current_room:
                return dest_room
        return None

    def to_dict(self) -> Dict:
        return {
            "rooms": [room.to_dict() for room in self.rooms],
            "current_room": self.current_room,
            "doors": self.doors
        }

    def save_to_file(self, filename: str):
        """Save the map to a JSON file in the format expected by the Rust game."""
        rust_format = self.convert_to_rust_format()
        
        with open(filename, 'w') as f:
            json.dump(rust_format, f, indent=2)
        print(f"Map saved to {filename}")

    @classmethod
    def load_from_file(cls, filename: str) -> 'Map':
        """Load a map from a JSON file."""
        with open(filename, 'r') as f:
            data = json.load(f)
        
        map_obj = cls()
        
        # Load rooms
        map_obj.rooms = []
        for room_data in data.get("rooms", []):
            room = Room(len(room_data[0]) if room_data else 12, len(room_data) if room_data else 12)
            
            # Convert numeric grid to TileType
            for y, row in enumerate(room_data):
                for x, tile_value in enumerate(row):
                    if tile_value == 0:
                        room.grid[y][x] = TileType.Empty
                    elif tile_value == 1:
                        room.grid[y][x] = TileType.Wall
                    elif tile_value == 2:
                        room.grid[y][x] = TileType.Wall2
                    elif tile_value == 3:
                        room.grid[y][x] = TileType.Wall3
                    elif tile_value == 4:
                        room.grid[y][x] = TileType.Wall4
                    elif tile_value == 5:
                        room.grid[y][x] = TileType.Wall5
                    elif tile_value == 6:
                        room.grid[y][x] = TileType.Wall6
                    elif tile_value == 7:
                        room.grid[y][x] = TileType.Door
                    else:
                        room.grid[y][x] = TileType.Empty
            
            map_obj.rooms.append(room)
        
        # If no rooms were loaded, create a default room
        if not map_obj.rooms:
            map_obj.rooms = [Room()]
        
        # Load current room
        map_obj.current_room = data.get("current_room", 0)
        
        # Load doors
        map_obj.doors = []
        for door_data in data.get("doors", []):
            if len(door_data) >= 3:
                x, y, dest_room = door_data
                # Find the source room by checking which room has a door at this position
                for room_idx, room in enumerate(map_obj.rooms):
                    if room_idx != dest_room and 0 <= x < room.width and 0 <= y < room.height and room.grid[y][x] == TileType.Door:
                        map_obj.doors.append((x, y, room_idx, dest_room))
                        break
        
        return map_obj

    def convert_to_rust_format(self) -> Dict:
        """Convert the map to the format expected by the Rust game."""
        # Convert room grids to numeric format
        room_layouts = []
        for room in self.rooms:
            layout = []
            for row in room.grid:
                layout_row = []
                for tile in row:
                    if tile == TileType.Empty:
                        layout_row.append(0)
                    elif tile == TileType.Wall:
                        layout_row.append(1)
                    elif tile == TileType.Wall2:
                        layout_row.append(2)
                    elif tile == TileType.Wall3:
                        layout_row.append(3)
                    elif tile == TileType.Wall4:
                        layout_row.append(4)
                    elif tile == TileType.Wall5:
                        layout_row.append(5)
                    elif tile == TileType.Wall6:
                        layout_row.append(6)
                    elif tile == TileType.Door:
                        layout_row.append(7)
                    else:
                        layout_row.append(0)  # Default to empty
                layout.append(layout_row)
            room_layouts.append(layout)
        
        # Convert doors to Rust format (x, y, destination_room)
        rust_doors = []
        for x, y, source_room, dest_room in self.doors:
            rust_doors.append((x, y, dest_room))
        
        return {
            "rooms": room_layouts,
            "current_room": self.current_room,
            "doors": rust_doors
        }

class MapEditor:
    def __init__(self, map_file: Optional[str] = None):
        self.screen = pygame.display.set_mode((WINDOW_WIDTH, WINDOW_HEIGHT))
        pygame.display.set_caption("Rust Game Map Editor")
        self.clock = pygame.time.Clock()
        
        # Load or create a map
        if map_file and os.path.exists(map_file):
            try:
                self.map = Map.load_from_file(map_file)
                print(f"Loaded map from {map_file}")
            except Exception as e:
                print(f"Error loading map: {e}")
                self.map = Map()
        else:
            self.map = Map()
        
        self.current_tile_type = TileType.Wall
        self.camera_x = 0
        self.camera_y = 0
        self.is_dragging = False
        self.drag_start_x = 0
        self.drag_start_y = 0
        self.show_grid = True
        self.show_help = False
        self.map_file = map_file or "new_map.json"
        
        # UI elements
        self.sidebar_width = 200
        self.tile_buttons = self._create_tile_buttons()
        self.room_buttons = self._create_room_buttons()
        
        # Font for text
        self.font = pygame.font.SysFont('Arial', 16)

    def _create_tile_buttons(self) -> List[Dict]:
        buttons = []
        y_pos = 50
        
        for tile_type in TileType:
            buttons.append({
                'rect': pygame.Rect(WINDOW_WIDTH - self.sidebar_width + 10, y_pos, 30, 30),
                'color': TILE_COLORS[tile_type],
                'tile_type': tile_type,
                'label': tile_type.name
            })
            y_pos += 40
        
        return buttons

    def _create_room_buttons(self) -> List[Dict]:
        buttons = []
        x_pos = WINDOW_WIDTH - self.sidebar_width + 10
        y_pos = 400
        
        # Add room button
        buttons.append({
            'rect': pygame.Rect(x_pos, y_pos, 180, 30),
            'color': GREEN,
            'action': 'add_room',
            'label': 'Add Room'
        })
        y_pos += 40
        
        # Room navigation buttons
        for i in range(len(self.map.rooms)):
            buttons.append({
                'rect': pygame.Rect(x_pos, y_pos, 180, 30),
                'color': BLUE if i == self.map.current_room else LIGHT_GRAY,
                'action': 'select_room',
                'room_index': i,
                'label': f'Room {i}'
            })
            y_pos += 40
        
        return buttons

    def update_room_buttons(self):
        self.room_buttons = self._create_room_buttons()

    def run(self):
        running = True
        
        while running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        running = False
                    elif event.key == pygame.K_s and pygame.key.get_mods() & pygame.KMOD_CTRL:
                        self.map.save_to_file(self.map_file)
                    elif event.key == pygame.K_g:
                        self.show_grid = not self.show_grid
                    elif event.key == pygame.K_h:
                        self.show_help = not self.show_help
                    elif event.key == pygame.K_1:
                        self.current_tile_type = TileType.Empty
                    elif event.key == pygame.K_2:
                        self.current_tile_type = TileType.Wall
                    elif event.key == pygame.K_3:
                        self.current_tile_type = TileType.Wall2
                    elif event.key == pygame.K_4:
                        self.current_tile_type = TileType.Wall3
                    elif event.key == pygame.K_5:
                        self.current_tile_type = TileType.Wall4
                    elif event.key == pygame.K_6:
                        self.current_tile_type = TileType.Wall5
                    elif event.key == pygame.K_7:
                        self.current_tile_type = TileType.Wall6
                    elif event.key == pygame.K_8:
                        self.current_tile_type = TileType.Skull
                    elif event.key == pygame.K_9:
                        self.current_tile_type = TileType.Door
                
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    if event.button == 1:  # Left click
                        # Check if clicking on a UI element
                        for button in self.tile_buttons:
                            if button['rect'].collidepoint(event.pos):
                                self.current_tile_type = button['tile_type']
                                break
                        
                        for button in self.room_buttons:
                            if button['rect'].collidepoint(event.pos):
                                if button['action'] == 'add_room':
                                    self.map.add_room()
                                    self.update_room_buttons()
                                elif button['action'] == 'select_room':
                                    self.map.current_room = button['room_index']
                                    self.update_room_buttons()
                                break
                        
                        # Check if clicking on the grid
                        if event.pos[0] < WINDOW_WIDTH - self.sidebar_width:
                            grid_x = (event.pos[0] - self.camera_x) // GRID_SIZE
                            grid_y = (event.pos[1] - self.camera_y) // GRID_SIZE
                            
                            # If placing a door, handle door connections
                            if self.current_tile_type == TileType.Door:
                                # Ask for destination room
                                dest_room = self._prompt_for_door_destination()
                                if dest_room is not None and dest_room != self.map.current_room:
                                    self.map.add_door(grid_x, grid_y, dest_room)
                            else:
                                # Place the selected tile
                                self.map.rooms[self.map.current_room].set_tile(grid_x, grid_y, self.current_tile_type)
                    
                    elif event.button == 3:  # Right click
                        # Start camera drag
                        self.is_dragging = True
                        self.drag_start_x, self.drag_start_y = event.pos
                
                elif event.type == pygame.MOUSEBUTTONUP:
                    if event.button == 3:  # Right click
                        self.is_dragging = False
                
                elif event.type == pygame.MOUSEMOTION:
                    if self.is_dragging:
                        # Move camera
                        dx = event.pos[0] - self.drag_start_x
                        dy = event.pos[1] - self.drag_start_y
                        self.camera_x += dx
                        self.camera_y += dy
                        self.drag_start_x, self.drag_start_y = event.pos
            
            # Draw everything
            self.draw()
            
            # Cap the frame rate
            self.clock.tick(FPS)
        
        pygame.quit()

    def draw(self):
        self.screen.fill(BLACK)
        
        # Draw the grid
        self._draw_grid()
        
        # Draw the current room
        self._draw_room()
        
        # Draw the sidebar
        self._draw_sidebar()
        
        # Draw help text if enabled
        if self.show_help:
            self._draw_help()
        
        pygame.display.flip()

    def _draw_grid(self):
        if not self.show_grid:
            return
        
        # Draw vertical grid lines
        for x in range(0, WINDOW_WIDTH - self.sidebar_width, GRID_SIZE):
            pygame.draw.line(self.screen, GRAY, (x + self.camera_x, 0), (x + self.camera_x, WINDOW_HEIGHT), 1)
        
        # Draw horizontal grid lines
        for y in range(0, WINDOW_HEIGHT, GRID_SIZE):
            pygame.draw.line(self.screen, GRAY, (0, y + self.camera_y), (WINDOW_WIDTH - self.sidebar_width, y + self.camera_y), 1)

    def _draw_room(self):
        room = self.map.rooms[self.map.current_room]
        
        # Draw tiles
        for y in range(room.height):
            for x in range(room.width):
                tile_type = room.grid[y][x]
                color = TILE_COLORS[tile_type]
                
                rect = pygame.Rect(
                    x * GRID_SIZE + self.camera_x,
                    y * GRID_SIZE + self.camera_y,
                    GRID_SIZE,
                    GRID_SIZE
                )
                
                pygame.draw.rect(self.screen, color, rect)
                
                # Draw tile borders
                pygame.draw.rect(self.screen, BLACK, rect, 1)
                
                # Draw special indicators for doors
                if tile_type == TileType.Door:
                    # Draw a circle in the center of the door
                    center_x = x * GRID_SIZE + self.camera_x + GRID_SIZE // 2
                    center_y = y * GRID_SIZE + self.camera_y + GRID_SIZE // 2
                    pygame.draw.circle(self.screen, YELLOW, (center_x, center_y), GRID_SIZE // 4)
                    
                    # Draw the destination room number
                    dest_room = self.map.get_door_destination(x, y)
                    if dest_room is not None:
                        text = self.font.render(str(dest_room), True, BLACK)
                        text_rect = text.get_rect(center=(center_x, center_y))
                        self.screen.blit(text, text_rect)
        
        # Draw decorations
        for x, y, tile_type in room.decorations:
            color = TILE_COLORS[tile_type]
            
            rect = pygame.Rect(
                x * GRID_SIZE + self.camera_x + GRID_SIZE // 4,
                y * GRID_SIZE + self.camera_y + GRID_SIZE // 4,
                GRID_SIZE // 2,
                GRID_SIZE // 2
            )
            
            pygame.draw.rect(self.screen, color, rect)

    def _draw_sidebar(self):
        # Draw sidebar background
        sidebar_rect = pygame.Rect(WINDOW_WIDTH - self.sidebar_width, 0, self.sidebar_width, WINDOW_HEIGHT)
        pygame.draw.rect(self.screen, LIGHT_GRAY, sidebar_rect)
        
        # Draw title
        title = self.font.render("Map Editor", True, BLACK)
        self.screen.blit(title, (WINDOW_WIDTH - self.sidebar_width + 10, 10))
        
        # Draw current room info
        room_info = self.font.render(f"Room: {self.map.current_room}", True, BLACK)
        self.screen.blit(room_info, (WINDOW_WIDTH - self.sidebar_width + 10, 30))
        
        # Draw tile buttons
        for button in self.tile_buttons:
            pygame.draw.rect(self.screen, button['color'], button['rect'])
            pygame.draw.rect(self.screen, BLACK, button['rect'], 1)
            
            # Highlight the selected tile type
            if button['tile_type'] == self.current_tile_type:
                pygame.draw.rect(self.screen, RED, button['rect'], 3)
            
            # Draw tile name
            label = self.font.render(button['label'], True, BLACK)
            self.screen.blit(label, (button['rect'].right + 10, button['rect'].centery - 8))
        
        # Draw room buttons
        for button in self.room_buttons:
            pygame.draw.rect(self.screen, button['color'], button['rect'])
            pygame.draw.rect(self.screen, BLACK, button['rect'], 1)
            
            # Draw button label
            label = self.font.render(button['label'], True, BLACK)
            self.screen.blit(label, (button['rect'].centerx - label.get_width() // 2, button['rect'].centery - 8))
        
        # Draw help hint
        help_hint = self.font.render("Press H for help", True, BLACK)
        self.screen.blit(help_hint, (WINDOW_WIDTH - self.sidebar_width + 10, WINDOW_HEIGHT - 30))

    def _draw_help(self):
        # Draw help overlay
        help_surface = pygame.Surface((WINDOW_WIDTH, WINDOW_HEIGHT), pygame.SRCALPHA)
        help_surface.fill((0, 0, 0, 200))
        
        # Help text
        help_text = [
            "Map Editor Controls:",
            "",
            "Left Click: Place selected tile",
            "Right Click + Drag: Move camera",
            "Ctrl+S: Save map",
            "G: Toggle grid",
            "H: Toggle help",
            "1-9: Select tile type",
            "Esc: Quit",
            "",
            "Tile Types:",
            "1: Empty",
            "2: Wall",
            "3: Wall2 (Left)",
            "4: Wall3 (Right)",
            "5: Wall4 (Bottom)",
            "6: Wall5 (Top Left)",
            "7: Wall6 (Top Right)",
            "8: Skull",
            "9: Door"
        ]
        
        y_pos = 50
        for line in help_text:
            text = self.font.render(line, True, WHITE)
            help_surface.blit(text, (WINDOW_WIDTH // 2 - text.get_width() // 2, y_pos))
            y_pos += 25
        
        self.screen.blit(help_surface, (0, 0))

    def _prompt_for_door_destination(self) -> Optional[int]:
        """Show a simple prompt to get the destination room for a door."""
        # This is a very basic implementation - in a real app, you'd want a proper UI dialog
        
        # Create a small dialog surface
        dialog_width, dialog_height = 300, 200
        dialog = pygame.Surface((dialog_width, dialog_height))
        dialog.fill(WHITE)
        pygame.draw.rect(dialog, BLACK, (0, 0, dialog_width, dialog_height), 2)
        
        # Title
        title_font = pygame.font.SysFont('Arial', 20, bold=True)
        title = title_font.render("Door Destination", True, BLACK)
        dialog.blit(title, (dialog_width // 2 - title.get_width() // 2, 20))
        
        # Instructions
        instructions = self.font.render("Enter destination room number:", True, BLACK)
        dialog.blit(instructions, (20, 60))
        
        # Available rooms
        room_text = self.font.render(f"Available rooms: 0-{len(self.map.rooms)-1}", True, BLACK)
        dialog.blit(room_text, (20, 90))
        
        # Input field
        input_rect = pygame.Rect(20, 120, 260, 30)
        pygame.draw.rect(dialog, LIGHT_GRAY, input_rect)
        pygame.draw.rect(dialog, BLACK, input_rect, 1)
        
        # Buttons
        ok_button = pygame.Rect(60, 160, 80, 30)
        cancel_button = pygame.Rect(160, 160, 80, 30)
        pygame.draw.rect(dialog, GREEN, ok_button)
        pygame.draw.rect(dialog, RED, cancel_button)
        pygame.draw.rect(dialog, BLACK, ok_button, 1)
        pygame.draw.rect(dialog, BLACK, cancel_button, 1)
        
        ok_text = self.font.render("OK", True, BLACK)
        cancel_text = self.font.render("Cancel", True, BLACK)
        dialog.blit(ok_text, (ok_button.centerx - ok_text.get_width() // 2, ok_button.centery - ok_text.get_height() // 2))
        dialog.blit(cancel_text, (cancel_button.centerx - cancel_text.get_width() // 2, cancel_button.centery - cancel_text.get_height() // 2))
        
        # Position the dialog in the center of the screen
        dialog_x = WINDOW_WIDTH // 2 - dialog_width // 2
        dialog_y = WINDOW_HEIGHT // 2 - dialog_height // 2
        
        # Input handling
        input_text = ""
        input_active = True
        dialog_active = True
        result = None
        
        while dialog_active:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    sys.exit()
                
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        dialog_active = False
                    elif event.key == pygame.K_RETURN:
                        try:
                            room_num = int(input_text)
                            if 0 <= room_num < len(self.map.rooms) and room_num != self.map.current_room:
                                result = room_num
                                dialog_active = False
                        except ValueError:
                            pass
                    elif event.key == pygame.K_BACKSPACE:
                        input_text = input_text[:-1]
                    else:
                        # Only allow digits
                        if event.unicode.isdigit() and len(input_text) < 3:
                            input_text += event.unicode
                
                elif event.type == pygame.MOUSEBUTTONDOWN:
                    if event.button == 1:  # Left click
                        mouse_pos = (event.pos[0] - dialog_x, event.pos[1] - dialog_y)
                        
                        if ok_button.collidepoint(mouse_pos):
                            try:
                                room_num = int(input_text)
                                if 0 <= room_num < len(self.map.rooms) and room_num != self.map.current_room:
                                    result = room_num
                                    dialog_active = False
                            except ValueError:
                                pass
                        
                        elif cancel_button.collidepoint(mouse_pos):
                            dialog_active = False
                        
                        elif input_rect.collidepoint(mouse_pos):
                            input_active = True
                        else:
                            input_active = False
            
            # Draw the main screen
            self.draw()
            
            # Draw input text
            if input_active:
                pygame.draw.rect(dialog, WHITE, input_rect)
                pygame.draw.rect(dialog, BLUE, input_rect, 2)
            else:
                pygame.draw.rect(dialog, LIGHT_GRAY, input_rect)
                pygame.draw.rect(dialog, BLACK, input_rect, 1)
            
            text_surface = self.font.render(input_text, True, BLACK)
            dialog.blit(text_surface, (input_rect.x + 5, input_rect.y + 5))
            
            # Draw the dialog
            self.screen.blit(dialog, (dialog_x, dialog_y))
            
            pygame.display.flip()
            self.clock.tick(FPS)
        
        return result

def main():
    parser = argparse.ArgumentParser(description='Map Editor for Rust Game')
    parser.add_argument('map_file', nargs='?', help='Path to the map file to edit')
    args = parser.parse_args()
    
    editor = MapEditor(args.map_file)
    editor.run()

if __name__ == "__main__":
    main() 