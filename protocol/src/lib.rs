#[derive(Debug, Clone)]
pub enum ProtocolError {
    ServerLineUnparsable,
    InvalidFormat(String),
}
pub mod zones;
use zones::ZoneLink;


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

impl std::str::FromStr for ServerToClient {
    type Err = ProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut msg = s.split(" ");
        let srv_conf = msg.next().unwrap();
        // let user_msg = msg.next().unwrap();

        let cmd = msg.next().unwrap();
        match cmd {
            "chat" => {
                let username: &str = msg.next().unwrap();
                let mut rest = String::new();
                for word in msg {
                    rest.push_str(&format!(" {}", word));
                }

                Ok(Self::ChatMessage(username.to_string(), rest))
            }
            "Facing" => Err(ProtocolError::ServerLineUnparsable),
            "Username" => Err(ProtocolError::ServerLineUnparsable),

            err => {
                log::error!("{}", err);

                Err(ProtocolError::ServerLineUnparsable)
            }
        }
    }
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

    Goto(ZoneLink),
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
            Goto(link) => format!("goto {}\r\n", link),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_creation() {
        let pos = Position::new(10, -5);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, -5);
    }

    #[test]
    fn test_facing_from_str() {
        assert_eq!("North".parse::<Facing>().unwrap(), Facing::North);
        assert_eq!("East".parse::<Facing>().unwrap(), Facing::East);
        assert_eq!("South".parse::<Facing>().unwrap(), Facing::South);
        assert_eq!("West".parse::<Facing>().unwrap(), Facing::West);
        // Test default case
        assert_eq!("Invalid".parse::<Facing>().unwrap(), Facing::South);
    }

    #[test]
    fn test_facing_display() {
        assert_eq!(Facing::North.to_string(), "North");
        assert_eq!(Facing::East.to_string(), "East");
        assert_eq!(Facing::South.to_string(), "South");
        assert_eq!(Facing::West.to_string(), "West");
    }

    #[test]
    fn test_server_to_client_parse() {
        // Test chat message parsing
        let chat_msg = "SERVER chat user1 Hello World!";
        if let Ok(ServerToClient::ChatMessage(username, message)) = chat_msg.parse() {
            assert_eq!(username, "user1");
            assert_eq!(message, " Hello World!");
        } else {
            panic!("Failed to parse chat message");
        }

        // Test error cases
        assert!("SERVER Facing North".parse::<ServerToClient>().is_err());
        assert!("SERVER Username test".parse::<ServerToClient>().is_err());
    }

    #[test]
    fn test_client_to_server_formatting() {
        // Test movement command
        let move_cmd = ClientToServer::AttemptPlayerMove(Position::new(100, 200));
        assert_eq!(move_cmd.as_line(), "pos 100 200\r\n");

        // Test facing command
        let face_cmd = ClientToServer::AttemptPlayerFacingChange(Facing::North);
        assert_eq!(face_cmd.as_line(), "face North\r\n");

        // Test authentication commands
        let register_cmd = ClientToServer::Register("player1".to_string(), "pass123".to_string());
        assert_eq!(register_cmd.as_line(), "register player1 pass123\r\n");

        let login_cmd = ClientToServer::Login("player1".to_string(), "pass123".to_string());
        assert_eq!(login_cmd.as_line(), "login player1 pass123\r\n");

        // Test chat command
        let chat_cmd = ClientToServer::ChatMessage("Hello everyone!".to_string());
        assert_eq!(chat_cmd.as_line(), "chat Hello everyone!\r\n");

        // Test username command
        let username_cmd = ClientToServer::SetUsername("newname".to_string());
        assert_eq!(username_cmd.as_line(), "username newname\r\n");

        // Test position command
        let pos_cmd = ClientToServer::SetPosition(50, 75);
        assert_eq!(pos_cmd.as_line(), "pos 50 75\r\n");
    }
}
