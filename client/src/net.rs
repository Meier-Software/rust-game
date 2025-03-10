use std::{
    io::{Read, Write},
    net::TcpStream,
};

use protocol::ClientToServer;

#[derive(Debug)]
pub enum NCError {
    SendError,
    NoNewData,
    ConnectionError(String),
}

pub struct NetClient {
    tcp: Option<TcpStream>,
    offline_mode: bool,
}

impl NetClient {
    pub fn new() -> Self {
        Self::new_with_mode(false)
    }

    pub fn new_offline() -> Self {
        Self::new_with_mode(true)
    }

    fn new_with_mode(offline_mode: bool) -> Self {
        if offline_mode {
            log::info!("Starting in offline mode");
            return Self {
                tcp: None,
                offline_mode: true,
            };
        }

        let addr = "game.ablecorp.us:45250";
        let stream = match TcpStream::connect(addr) {
            Ok(stream) => {
                log::info!("Connected to server at {}", addr);
                // Set non-blocking mode
                let ret = stream.set_nonblocking(true);
                match ret {
                    Ok(_) => {}
                    Err(err) => {
                        log::error!(
                            "Failed to set nonblocking for the following reason {}.",
                            err
                        );
                    }
                }

                Some(stream)
            }
            Err(e) => {
                log::warn!("Failed to connect to server: {}. Switching to offline mode.", e);
                return Self {
                    tcp: None,
                    offline_mode: true,
                };
            }
        };

        Self { 
            tcp: stream,
            offline_mode: false,
        }
    }

    pub fn is_offline(&self) -> bool {
        self.offline_mode
    }

    pub fn send(&mut self, cts: ClientToServer) -> Result<(), NCError> {
        if self.offline_mode {
            // In offline mode, we just log the message and return success
            log::debug!("Offline mode: Would send {:?}", cts);
            return Ok(());
        }
        
        // Online mode - send to server
        self.send_str(cts.as_line())
    }

    pub fn send_str(&mut self, string: String) -> Result<(), NCError> {
        if self.offline_mode {
            // In offline mode, we just log the message and return success
            log::debug!("Offline mode: Would send {}", string.trim());
            return Ok(());
        }
        
        log::trace!("Sending: {}", string.trim());
        let a = string.to_string();
        match self.tcp.as_mut() {
            Some(stream) => {
                match stream.write(a.as_bytes()) {
                    Ok(bytes) => {
                        log::trace!("Sent {} bytes to server.", bytes);
                        Ok(())
                    }
                    Err(e) => {
                        log::error!("Error sending data: {}", e);
                        Err(NCError::SendError)
                    }
                }
            }
            None => Err(NCError::ConnectionError("Server connection lost".to_string())),
        }
    }

    pub fn recv(&mut self) -> Result<String, NCError> {
        if self.offline_mode {
            // In offline mode, we always return NoNewData
            // This prevents the game from waiting for server responses
            return Err(NCError::NoNewData);
        }
        
        let mut buffer = [0; 1024];
        use NCError::*;
        match self.tcp.as_mut() {
            Some(stream) => {
                // Set non-blocking mode
                stream.set_nonblocking(true).unwrap();
                
                match stream.read(&mut buffer) {
                    Ok(0) => Err(ConnectionError("Server closed connection".to_string())),
                    Ok(n) => {
                        let data = String::from_utf8_lossy(&buffer[0..n]).to_string();
                        Ok(data)
                    }
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Err(NCError::NoNewData),
                    Err(e) => Err(ConnectionError(e.to_string())),
                }
            }
            None => Err(ConnectionError("Server connection lost".to_string())),
        }
    }

    pub fn parse_server_message(&self, message: &str) -> Option<protocol::ServerToClient> {
        // Extract the username from the message if it contains "USR-"
        let username_from_prefix = message.split_whitespace()
            .find(|part| part.starts_with("USR-(") && part.ends_with("):"))
            .map(|part| part.trim_start_matches("USR-(").trim_end_matches("):").to_string());
        
        if message.contains("player_moved") {
            let parts: Vec<&str> = message.split_whitespace().collect();
            
            // Find the position of "player_moved" in the message
            if let Some(pos_index) = parts.iter().position(|&part| part == "player_moved") {
                // Check if we have enough parts after "player_moved"
                if pos_index + 4 < parts.len() {
                    // The format is now "player_moved username x y facing"
                    let username = parts[pos_index + 1].to_string();
                    
                    if let (Ok(x), Ok(y)) = (parts[pos_index + 2].parse::<i32>(), parts[pos_index + 3].parse::<i32>()) {
                        let facing_str = parts[pos_index + 4];
                        let facing = match facing_str {
                            "North" => protocol::Facing::North,
                            "East" => protocol::Facing::East,
                            "South" => protocol::Facing::South,
                            "West" => protocol::Facing::West,
                            _ => protocol::Facing::South,
                        };
                        
                        return Some(protocol::ServerToClient::PlayerMoved(
                            username,
                            protocol::Position::new(x, y),
                            facing,
                        ));
                    }
                }
            }
        } else if message.contains("player_joined") {
            let parts: Vec<&str> = message.split_whitespace().collect();
            
            // Find the position of "player_joined" in the message
            if let Some(pos_index) = parts.iter().position(|&part| part == "player_joined") {
                // Check if we have enough parts after "player_joined"
                if pos_index + 4 < parts.len() {
                    // The format is now "player_joined username x y facing"
                    let username = parts[pos_index + 1].to_string();
                    
                    if let (Ok(x), Ok(y)) = (parts[pos_index + 2].parse::<i32>(), parts[pos_index + 3].parse::<i32>()) {
                        let facing_str = parts[pos_index + 4];
                        let facing = match facing_str {
                            "North" => protocol::Facing::North,
                            "East" => protocol::Facing::East,
                            "South" => protocol::Facing::South,
                            "West" => protocol::Facing::West,
                            _ => protocol::Facing::South,
                        };
                        
                        return Some(protocol::ServerToClient::PlayerJoined(
                            username,
                            protocol::Position::new(x, y),
                            facing,
                        ));
                    }
                }
            }
        } else if message.contains("player_left") {
            let parts: Vec<&str> = message.split_whitespace().collect();
            
            // Find the position of "player_left" in the message
            if let Some(pos_index) = parts.iter().position(|&part| part == "player_left") {
                // Check if we have enough parts after "player_left"
                if pos_index + 1 < parts.len() {
                    // The format is "player_left username"
                    let username = parts[pos_index + 1].to_string();
                    return Some(protocol::ServerToClient::PlayerLeft(username));
                }
            }
        }
        
        None
    }
}
