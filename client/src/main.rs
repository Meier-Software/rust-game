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

    // Check for offline mode argument
    let args: Vec<String> = env::args().collect();
    let offline_mode = args.iter().any(|arg| arg == "--offline" || arg == "-o");

    // Print a message about offline mode if not specified
    if !offline_mode {
        println!("Note: You can run in offline mode with '--offline' or '-o' flag");
        println!("Example: cargo r --bin client -- --offline");
    }

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
    
    // Create game state based on mode
    let state = if offline_mode {
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
}
