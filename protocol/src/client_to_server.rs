use crate::{Facing, Position};

#[derive(Debug)]
pub enum ClientToServer {
    AttemptPlayerMove(Position),
    AttemptPlayerFacingChange(Facing),

    Register(String, String),
    Login(String, String),
    ChatMessage(String),
    SetUsername(String),
    SetPosition(i32, i32),
    Command(Vec<String>),
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
                for x in items {
                    stri.push_str(x.as_str());
                }

                format!("{}", stri)
            }
        }
    }
}
