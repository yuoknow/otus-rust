use std::io;

use crate::client::Client;
use crate::command::Command;

mod client;
mod command;
mod response;

fn main() {
    let mut client = Client::connect("127.0.0.1:55331").expect("Connection error");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = match input.trim() {
            "1" => Command::TurnOn,
            "2" => Command::TurnOff,
            "3" => Command::Info,
            _ => break,
        };
        let response = client.send(command);
        println!("{:?}", response);
    }
    println!("Client disconnected");
}
