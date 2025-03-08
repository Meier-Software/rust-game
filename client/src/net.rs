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
    tcp: TcpStream,
}

impl NetClient {
    pub fn new() -> Self {
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

                stream
            }
            Err(e) => {
                panic!("Failed to connect to server: {}", e);
            }
        };

        Self { tcp: stream }
    }

    pub fn send(&mut self, cts: ClientToServer) -> Result<(), NCError> {
        self.send_str(cts.as_line())
    }

    pub fn send_str(&mut self, string: String) -> Result<(), NCError> {
        log::trace!("Sending: {}", string.trim());
        let a = string.to_string();
        match self.tcp.write(a.as_bytes()) {
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

    pub fn recv(&mut self) -> Result<String, NCError> {
        let mut buffer = [0; 1024];
        use NCError::*;
        match self.tcp.read(&mut buffer) {
            Ok(0) => Err(ConnectionError("Server closed connection".to_string())),
            Ok(n) => {
                let data = String::from_utf8_lossy(&buffer[0..n]).to_string();
                Ok(data)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Err(NCError::NoNewData),
            Err(e) => Err(ConnectionError(e.to_string())),
        }
    }
}
