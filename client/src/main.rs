// main.rs - Entry point for the game client
use std::path::PathBuf;
use ggez::{Context, GameResult, event};

mod net;
mod assets;
mod input;
mod player;
mod map;
mod game_state;

use game_state::GameState;

pub fn main() -> GameResult {
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
