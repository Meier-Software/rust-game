use std::{io::Write, net::TcpStream};

pub struct NetClient {
    tcp: TcpStream,
}

impl NetClient {
    pub fn new() -> Self {
        let addr = "game.ablecorp.us:45250";
        let stream = TcpStream::connect(addr).unwrap();

        Self { tcp: stream }
    }
    pub fn send(&mut self, string: String) {
        println!("Net event sent.");
        let a = format!("{}", string);
        let _ = self.tcp.write(a.as_bytes());
    }
}
