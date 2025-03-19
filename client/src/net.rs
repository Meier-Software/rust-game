use std::{
    io::{Read, Write},
    net::TcpStream,
    str::FromStr,
};

use protocol::ClientToServer;

/// Network client errors that can occur during communication
#[derive(Debug)]
pub enum NCError {
    /// Failed to send data to the server
    SendError,
    /// No new data available from the server
    NoNewData,
    /// Connection-related errors with descriptive message
    ConnectionError(String),
}

/// Network client that handles communication with the game server
pub struct NetClient {
    tcp: Option<TcpStream>,
    offline_mode: bool,
}

impl Default for NetClient {
    fn default() -> Self {
        Self::new()
    }
}

impl NetClient {
    /// Creates a new network client in online mode
    pub fn new() -> Self {
        Self::new_with_mode(false)
    }

    /// Creates a new network client in offline mode
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
                if let Err(err) = stream.set_nonblocking(true) {
                    log::error!("Failed to set nonblocking mode: {}", err);
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

    /// Returns whether the client is in offline mode
    pub fn is_offline(&self) -> bool {
        self.offline_mode
    }

    /// Sends a client-to-server message
    pub fn send(&mut self, cts: ClientToServer) -> Result<(), NCError> {
        if self.offline_mode {
            log::debug!("Offline mode: Would send {:?}", cts);
            return Ok(());
        }

        self.send_str(cts.as_line())
    }

    /// Sends a raw string message to the server
    pub fn send_str(&mut self, string: String) -> Result<(), NCError> {
        if self.offline_mode {
            log::debug!("Offline mode: Would send {}", string.trim());
            return Ok(());
        }

        log::trace!("Sending: {}", string.trim());
        match self.tcp.as_mut() {
            Some(stream) => match stream.write(string.as_bytes()) {
                Ok(bytes) => {
                    log::trace!("Sent {} bytes to server", bytes);
                    Ok(())
                }
                Err(e) => {
                    log::error!("Error sending data: {}", e);
                    Err(NCError::SendError)
                }
            },
            None => Err(NCError::ConnectionError("Server connection lost".to_string())),
        }
    }

    /// Receives data from the server
    pub fn recv(&mut self) -> Result<String, NCError> {
        if self.offline_mode {
            return Err(NCError::NoNewData);
        }

        let mut buffer = [0; 1024];
        match self.tcp.as_mut() {
            Some(stream) => {
                stream.set_nonblocking(true).unwrap();

                match stream.read(&mut buffer) {
                    Ok(0) => Err(NCError::ConnectionError("Server closed connection".to_string())),
                    Ok(n) => Ok(String::from_utf8_lossy(&buffer[0..n]).to_string()),
                    Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        Err(NCError::NoNewData)
                    }
                    Err(e) => Err(NCError::ConnectionError(e.to_string())),
                }
            }
            None => Err(NCError::ConnectionError("Server connection lost".to_string())),
        }
    }

    /// Parses a server message into a ServerToClient enum
    pub fn parse_server_message(&self, message: &str) -> Option<protocol::ServerToClient> {
        log::trace!("Parsing server message: {}", message);
        protocol::ServerToClient::from_str(message).ok()
    }
}
