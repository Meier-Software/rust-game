# Simple 2D Game

A simple 2D game built with Rust using the GGEZ game engine. The game features a professor character navigating through a walled environment with smooth animations and dialogue system.

## Features

- Smooth character movement with arrow key controls
- Animated sprite-based character with walking animations in four directions
- Collision detection with walls
- Camera system that follows the player with zoom
- Interactive dialogue system (toggle with Enter key)
- Clean, pixel-art style graphics

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- OS: Windows, macOS, or Linux
- Graphics: Any GPU that supports basic 2D graphics

## Dependencies

- GGEZ: Game development framework
- Glam: Math library for game development

## Installation

For detailed installation instructions for your specific operating system, please see [INSTALL.md](INSTALL.md).

Quick start for experienced users:

1. Install Rust and required system dependencies (see INSTALL.md)
2. Clone the repository:
```bash
git clone https://github.com/hcm444/rust-game
cd rust-game-main
```

3. Ensure the resources directory is properly set up:
```
./resources/
└── sprites/
    ├── player/
    │   └── professor_walk_cycle_no_hat.png
    └── tiles/
        └── wall.png
```

4. Build and run:
```bash
cargo run
```

## Controls

- **Arrow Keys**: Move the character (Up, Down, Left, Right)
- **Enter**: Toggle dialogue box
- Close window to exit the game

## Game Mechanics

- The player controls a professor character in a walled environment
- The character animates smoothly while walking in any direction
- Collision detection prevents walking through walls
- The camera follows the player with a zoomed view
- A dialogue system can be toggled to show the character's thoughts

## Technical Details

- Window Size: 800x600 pixels
- Player Size: 32x32 pixels
- Grid Size: 64x64 pixels
- World Size: 800x800 pixels
- Camera Zoom: 2x
