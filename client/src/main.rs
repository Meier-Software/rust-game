// main.rs - Entry point for the game client
use ggez::{Context, GameResult, event};
use std::path::PathBuf;

mod assets;
mod game_state;
mod input;
mod map;
mod net;
mod player;

use game_state::GameState;

use simple_logger::SimpleLogger;

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

    simp_log.init().unwrap();
    log::info!("TESTING LOGGER.");

    let resource_dir = PathBuf::from("./client/assets");
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
