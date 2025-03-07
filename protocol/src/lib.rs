use specs::{prelude::*, Component};


#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}


pub enum Facing {
    North,
    East,
    South,
    West,
}

pub enum ServerToClient {
    EntityMoved(EntityId, Position),
}

pub enum ClientToServer {
    AttemptPlayerMove(Position),
    AttemptPlayerFacingChange(Facing)
}



pub fn init_world()->World{
     let mut world = World::new();
     


     world
}
