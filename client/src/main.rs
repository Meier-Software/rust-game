use std::path::PathBuf;

use ggez::{
    Context, GameResult, event,
    graphics::{self, Color, DrawParam, Image, Rect},
    mint::Point2,
};
use net::NetClient;
use protocol::Position;

const PLAYER_SIZE: f32 = 32.0;
const MOVEMENT_SPEED: f32 = 2.0;
const GRID_SIZE: f32 = 64.0;
const WORLD_SIZE: f32 = 800.0; // Smaller play area
const SPRITE_SHEET_WIDTH: f32 = 64.0; // Width of each sprite in the sheet
const SPRITE_SHEET_HEIGHT: f32 = 64.0; // Height of each sprite in the sheet
const ANIMATION_FRAME_TIME: f32 = 0.05; // Halved from 0.1 to make animation twice as fast
const CAMERA_ZOOM: f32 = 2.0; // Camera zoom factor (higher = more zoomed in)
const DIALOGUE_PADDING: f32 = 20.0;
const DIALOGUE_HEIGHT: f32 = 150.0;

mod net;

pub enum Stage {
    PreAuth,
    InMenu,
    InGame,
}

pub struct GameState {
    stage: Stage,
    nc: NetClient,

    sp: Image,
    pos: Position,
}

impl GameState {
    pub fn new(ctx: &mut ggez::Context) -> Self {
        let nc = NetClient::new();

        let player_sprite =
            Image::from_path(ctx, "/sprites/tiles/wall.png").expect("Failed to load wall sprite");
        let pos = Position::new(10.0, 0.0);

        Self {
            stage: Stage::PreAuth,
            nc,
            pos,
            sp: player_sprite,
        }
    }

    pub fn run_stage(&mut self) {
        match self.stage {
            Stage::PreAuth => {
                // println!("Pre Auth");
                // self.stage = Stage::InGame;
            }
            Stage::InMenu => {}
            Stage::InGame => {}
        }
    }

    pub fn draw_stage(&mut self, ctx: &mut Context) {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        match self.stage {
            Stage::PreAuth => {
                let offset = [self.pos.x, self.pos.y];
                let src_rect = Rect::new(self.pos.x, self.pos.y, 64.0, 64.0);

                let screen_width = ctx.gfx.window().inner_size().width as f32;
                let screen_height = ctx.gfx.window().inner_size().height as f32;

                let zoomed_width = screen_width / CAMERA_ZOOM;
                let zoomed_height = screen_height / CAMERA_ZOOM;
                
                canvas.set_screen_coordinates(Rect::new(0.0, 0.0, zoomed_width, zoomed_height));

                let draw_params = DrawParam::default()
                    .offset(offset)
                    .src(src_rect)
                    .scale([64.0, 64.0]);

                canvas.draw(&self.sp, draw_params);
            }
            Stage::InMenu => {}
            Stage::InGame => {}
        }

        canvas.finish(ctx).unwrap();
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    // Update once per tick.
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.run_stage();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.draw_stage(ctx);
        Ok(())
    }
}

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
