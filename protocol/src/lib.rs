use specs::{Component, VecStorage, World, WorldExt};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub fn world_register(world: &mut World) {
    world.register::<Position>();
    world.register::<Facing>();
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub enum Facing {
    North,
    East,
    South,
    West,
}

pub enum ServerToClient {
    EntityMoved(Position),
}

pub enum ClientToServer {
    AttemptPlayerMove(Position),
    AttemptPlayerFacingChange(Facing),
}

pub fn init_world() -> World {
    let mut world = World::new();

    world
}
