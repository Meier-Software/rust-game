pub struct Position {
    pub x: f32,
    pub y: f32,
}
impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

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

    Register(String, String),
}

impl ClientToServer {
    pub fn as_line(&self) -> String {
        use ClientToServer::*;
        let line = match self {
            AttemptPlayerMove(position) => format!("move {} {}\r\n", position.x, position.y),
            AttemptPlayerFacingChange(facing) => todo!(),
            Register(username, password) => format!("register {} {}\r\n", username, password),
        };
        line
    }
}
