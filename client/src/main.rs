// main.rs - Entry point for the game client
use game_state::GameState;
use ggez::{Context, GameResult, event};
use simple_logger::SimpleLogger;

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

    let a = filter::Filters::new();
    let _ = a.filter_to_wimble_text("abc shit".to_string());

    // Use the correct resource path based on the current directory
    let resource_dir = if std::path::Path::new("./assets").exists() {
        log::warn!("Run the client in the proper directory please :)");
        "./assets"
    } else {
        "./client/assets"
    };

    let cb = ggez::ContextBuilder::new("simple_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Simple 2D Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx);

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
