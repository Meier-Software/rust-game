// main.rs - Entry point for the game client
use game_state::GameState;
use ggez::{Context, GameResult, event};
use simple_logger::SimpleLogger;
use std::env;

mod assets;
mod filter;
mod game_state;
mod input;
mod map;
mod net;
mod player;

pub fn main() -> GameResult {
    let mut simp_log = SimpleLogger::new();
    let ignored_modules = vec![
        "gilrs_core",
        "gilrs",
        "naga",
        "ggez",
        "wgpu_hal",
        "wgpu_core",
        "winit",
        "mio",
    ];

    for module in ignored_modules {
        simp_log = simp_log.with_module_level(module, log::LevelFilter::Warn);
    }

    simp_log.with_level(log::LevelFilter::Info).init().unwrap();

    // Parse command line arguments
    let args: Vec<String> = env::args().collect();
    let offline_mode = args.iter().any(|arg| arg == "--offline" || arg == "-o");

    // Check for export map argument
    let export_map = args
        .iter()
        .position(|arg| arg == "--export-map" || arg == "-e");

    // If export map argument is provided, create a map and export it to JSON
    if let Some(pos) = export_map {
        // Get the output path, default to "map.json" if not provided
        let output_path = if pos + 1 < args.len() && !args[pos + 1].starts_with('-') {
            &args[pos + 1]
        } else {
            "map.json"
        };

        println!("Exporting map to {}", output_path);
        let map = map::Map::new();
        if let Err(e) = map.to_json(output_path) {
            eprintln!("Error exporting map: {}", e);
            std::process::exit(1);
        }
        println!("Map exported successfully!");
        std::process::exit(0);
    }

    // Check for import map argument
    let import_map = args
        .iter()
        .position(|arg| arg == "--import-map" || arg == "-i");

    // Store the path to the map file if import is requested
    let map_path = if let Some(pos) = import_map {
        if pos + 1 < args.len() && !args[pos + 1].starts_with('-') {
            println!("Will import map from {}", &args[pos + 1]);
            Some(args[pos + 1].clone())
        } else {
            eprintln!("Error: --import-map requires a file path");
            std::process::exit(1);
        }
    } else {
        None
    };

    // Print a message about offline mode if not specified
    if !offline_mode {
        println!("Note: You can run in offline mode with '--offline' or '-o' flag");
        println!("Example: cargo r --bin client -- --offline");
    }

    // Print a message about exporting and importing the map
    println!("Note: You can export the map to JSON with '--export-map' or '-e' flag");
    println!("Example: cargo r --bin client -- --export-map [output_path]");
    println!("Note: You can import a map from JSON with '--import-map' or '-i' flag");
    println!("Example: cargo r --bin client -- --import-map [input_path]");
    println!("Note: The game will use the default map from assets/default_map.json if available");

    let a = filter::Filters::new();
    let _ = a.filter_to_wimble_text("abc shit".to_string());

    // Use the correct resource path based on the current directory
    let resource_dir = if std::path::Path::new("./assets").exists() {
        log::warn!("Run the client in the proper directory please :)");
        "./assets"
    } else {
        "./client/assets"
    };

    // Set window title based on mode
    let window_title = if offline_mode {
        "Simple 2D Game (Offline Mode)"
    } else {
        "Simple 2D Game"
    };

    let cb = ggez::ContextBuilder::new("simple_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title(window_title))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;

    // Create game state based on mode and map import
    let state = if let Some(path) = map_path {
        // Import map and create game state
        match map::Map::from_json(&path) {
            Ok(imported_map) => {
                log::info!("Successfully imported map from {}", path);
                if offline_mode {
                    GameState::new_offline_with_map(&mut ctx, imported_map)
                } else {
                    GameState::new_with_map(&mut ctx, imported_map)
                }
            }
            Err(e) => {
                eprintln!("Error importing map: {}", e);
                std::process::exit(1);
            }
        }
    } else if offline_mode {
        log::info!("Starting game in offline mode");
        GameState::new_offline(&mut ctx)
    } else {
        log::info!("Starting game in online mode");
        GameState::new(&mut ctx)
    };

    event::run(ctx, event_loop, state)
}

// Implement the EventHandler trait for GameState
impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        self.update(ctx)
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), ggez::GameError> {
        self.draw(ctx)
    }

    fn text_input_event(
        &mut self,
        _ctx: &mut Context,
        character: char,
    ) -> Result<(), ggez::GameError> {
        self.handle_text_input(_ctx, character)
    }
}
