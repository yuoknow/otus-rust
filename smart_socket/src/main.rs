use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::handler::RequestHandler;
use crate::smart_socket::SmartSocket;

mod handler;
pub mod smart_socket;

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:55331");
    let tcp = TcpListener::bind(addr)
        .await
        .expect("Cant bind tcp listener");
    let request_handler = Arc::new(Mutex::new(RequestHandler::new(SmartSocket::default())));

    while let Ok((mut stream, addr)) = tcp.accept().await {
        println!("New connection {}", addr);
        let request_handler = request_handler.clone();
        tokio::spawn(async move {
            let mut buff = [0u8];
            while stream.read_exact(&mut buff).await.is_ok() {
                let response = request_handler.lock().await.handle(buff[0]);
                println!("Response: {:?}", response);
                let response_buf: [u8; 5] = response.into();
                if stream.write_all(&response_buf).await.is_err() {
                    break;
                }
            }
        });
    }
}
