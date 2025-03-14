#[derive(Debug, Copy, Clone)]
pub enum ProtocolError{

}

// This is a teleportation link to be used by doors. hub@x20y30
pub struct ZoneLink {
    // A slash seperated list.
    pub zone: String,
    pub pos: Position,
}

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

impl std::str::FromStr for Facing {
    type Err = ProtocolError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "North" => Ok(Self::North),
            "East" => Ok(Self::East),
            "South" => Ok(Self::South),
            "West" => Ok(Self::West),
            _ => Ok(Self::South),
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
    // TODO: Remove pos and facing from here and move it to the above
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
    Command(Vec<String>)
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
            Command(items) => {
                let mut stri = String::new();
                for x in items{
                    stri.push_str(x.as_str());
            };

            format!("{}", stri)
        }
        }
    }
}
