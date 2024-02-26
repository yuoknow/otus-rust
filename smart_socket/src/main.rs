use std::io::{Read, Write};
use std::net::TcpListener;

use crate::handler::RequestHandler;
use crate::smart_socket::SmartSocket;

mod handler;
pub mod smart_socket;

fn main() {
    let addr = String::from("127.0.0.1:55331");
    let tcp = TcpListener::bind(addr).expect("Cant bind tcp listener");
    let mut request_handler = RequestHandler::new(SmartSocket::default());

    while let Some(connection) = tcp.incoming().next() {
        let mut conn = match connection {
            Ok(conn) => {
                println!("New connection");
                conn
            }
            Err(err) => {
                println!("Connection error: {}", err);
                continue;
            }
        };
        let mut buff = [0u8];
        while conn.read_exact(&mut buff).is_ok() {
            let response = request_handler.handle(buff[0]);
            println!("Response: {:?}", response);
            let response_buf: [u8; 5] = response.into();
            if conn.write_all(&response_buf).is_err() {
                break;
            }
        }
    }
}
