use std::error::Error;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

use crate::command::Command;
use crate::response::Response;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub fn connect<Addrs>(addrs: Addrs) -> Result<Self, Box<dyn Error>>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Ok(Self { stream })
    }

    pub fn send(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()])?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer.into())
    }
}
