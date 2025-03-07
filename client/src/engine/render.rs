use ggez::Context;
use protocol::{Facing, Position};
use specs::{ReadStorage, System, WriteStorage};
use specs::Join;

use super::{Engine, net::NetClient};

impl Engine {
    // Updated once per frame. FPSRate
    pub fn fps_update(&mut self, ctx: &mut Context) {
        // TODO: Render a frame here.

        // TODO: Get player position & direction.

        // TODO: Render img correct direction.
    }
}

pub struct RenderFrame;

impl<'ecs_life> System<'ecs_life> for RenderFrame {
    // SystemData is what you are requesting from the world.
    type SystemData = (
        ReadStorage<'ecs_life, Position>,
        ReadStorage<'ecs_life, Facing>,

    );

    /// Render a frame here.
    fn run(&mut self, data: Self::SystemData) {
        let (position, facing) = data;

        // Get player position & direction.
        for pos in (&position, &facing).join() {
            // TODO: Render img correct direction.

        }

    }
}
