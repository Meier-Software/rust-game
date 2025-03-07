use std::path::PathBuf;

use ggez::{event, GameResult};
use net::NetClient;


mod net;


pub enum Stage{
 PreAuth,
 InMenu,
 InGame,
}


pub struct GameState {
 stage: Stage,
 nc: NetClient,
}


impl GameState {
 pub fn new(_ctx: &mut ggez::Context) -> Self{
  let nc = NetClient::new();

  Self { stage: Stage::PreAuth, nc }
 }



 pub fn run_stage(&mut self) {
  match self.stage{
    Stage::PreAuth => {
     
    },
    Stage::InMenu => {
     
    },
    Stage::InGame => {
     
    },
  }
 }
}




impl event::EventHandler<ggez::GameError> for GameState {
    // Update once per tick.
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        Ok(())
    }
}

pub fn main()  -> GameResult{

    let resource_dir = PathBuf::from("./client/assets");
    let cb = ggez::ContextBuilder::new("simple_game", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Simple 2D Game"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
        .add_resource_path(resource_dir);

    let (mut ctx, event_loop) = cb.build()?;
    let state = GameState::new(&mut ctx);

    event::run(ctx, event_loop, state)


}
