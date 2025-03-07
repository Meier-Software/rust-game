use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[derive(Debug)]
pub enum NCError {
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
                println!("Connected to server at {}", addr);
                // Set non-blocking mode
                let _ = stream.set_nonblocking(true);
                stream
            }
            Err(e) => {
                panic!("Failed to connect to server: {}", e);
            }
        };

        Self { tcp: stream }
    }

    pub fn send(&mut self, string: String) {
        println!("Sending: {}", string.trim());
        let a = format!("{}", string);
        match self.tcp.write(a.as_bytes()) {
            Ok(_) => {}
            Err(e) => println!("Error sending data: {}", e),
        }
    }

    pub fn recv(&mut self) -> Result<String, NCError> {
        let mut buffer = [0; 1024];

        match self.tcp.read(&mut buffer) {
            Ok(0) => Err(NCError::ConnectionError(
                "Server closed connection".to_string(),
            )),
            Ok(n) => {
                let data = String::from_utf8_lossy(&buffer[0..n]).to_string();
                Ok(data)
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => Err(NCError::NoNewData),
            Err(e) => Err(NCError::ConnectionError(e.to_string())),
        }
    }
}
