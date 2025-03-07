use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

#[derive(Debug)]
pub enum NCError {
    NoNewData,
}

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

    pub fn recv(&mut self) -> Result<String, NCError> {
        let mut string = String::new();

        let dur: Duration = Duration::new(0, 1000);

        let ret_to = self.tcp.set_read_timeout(Some(dur));
        // println!("{:?}", ret_to);

        let ret = self.tcp.read_to_string(&mut string);
        match ret {
            Ok(a) => Ok(string),
            Err(err) => Err(NCError::NoNewData),
        }
    }
}
