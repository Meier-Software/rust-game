use super::Engine;

impl ggez::event::EventHandler<ggez::GameError> for Engine {
    fn update(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        // TODO: Get input from ctx.
        self.fixed_update();
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut ggez::Context) -> Result<(), ggez::GameError> {
        self.fps_update();
        Ok(())
    }
}
