#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}
impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Facing {
    North,
    East,
    South,
    West,
}
impl std::fmt::Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Facing::*;
        let a = match self {
            North => "North".to_string(),
            East => "Eorth".to_string(),
            South => "Sorth".to_string(),
            West => "Worth".to_string(),
        };
        write!(f, "{}", a)
    }
}

pub enum ServerToClient {
    EntityMoved(Position),
    PlayerJoined(String, Position, Facing),
    PlayerLeft(String),
    PlayerMoved(String, Position, Facing),
}

#[derive(Debug)]
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
            AttemptPlayerFacingChange(facing) => format!("face {}\r\n", facing),
            Register(username, password) => format!("register {} {}\r\n", username, password),
        };
        line
    }
}
