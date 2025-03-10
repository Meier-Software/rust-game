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

impl Facing {
    pub fn from_str(stri: &str) -> Self {
        match stri {
            "North" => Self::North,
            "East" => Self::East,
            "South" => Self::South,
            "West" => Self::West,
            _ => Self::South,
        }
    }
}

impl std::fmt::Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Facing::*;
        let a = match self {
            North => "North".to_string(),
            East => "East".to_string(),
            South => "South".to_string(),
            West => "West".to_string(),
        };
        write!(f, "{}", a)
    }
}

#[derive(Debug)]
pub enum ServerToClient {
    EntityMoved(Position),
    PlayerJoined(String, Position, Facing),
    PlayerLeft(String),
    PlayerMoved(String, Position, Facing),
    ChatMessage(String, String),
}

#[derive(Debug)]
pub enum ClientToServer {
    AttemptPlayerMove(Position),
    AttemptPlayerFacingChange(Facing),

    Register(String, String),
    Login(String, String),
    ChatMessage(String),
    SetUsername(String),
    SetPosition(i32, i32),
}

impl ClientToServer {
    pub fn as_line(&self) -> String {
        use ClientToServer::*;

        match self {
            AttemptPlayerMove(position) => format!("pos {} {}\r\n", position.x, position.y),
            AttemptPlayerFacingChange(facing) => format!("face {}\r\n", facing),
            Register(username, password) => format!("register {} {}\r\n", username, password),
            Login(username, password) => format!("login {} {}\r\n", username, password),
            ChatMessage(message) => format!("chat {}\r\n", message),
            SetUsername(username) => format!("username {}\r\n", username),
            SetPosition(x, y) => format!("pos {} {}\r\n", x, y),
        }
    }
}
