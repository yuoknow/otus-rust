use std::error::Error;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpStream, ToSocketAddrs};

use crate::command::Command;
use crate::response::Response;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn connect<Addrs>(addrs: Addrs) -> Result<Self, Box<dyn Error>>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs).await?;
        Ok(Self { stream })
    }

    pub async fn send(&mut self, command: Command) -> Result<Response, Box<dyn Error>> {
        self.stream.write_all(&[command.into()]).await?;
        let mut buffer = [0u8; 5];
        self.stream.read_exact(&mut buffer).await?;
        Ok(buffer.into())
    }
}
