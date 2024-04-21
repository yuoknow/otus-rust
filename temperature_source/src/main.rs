use std::str::FromStr;
use std::{net::SocketAddr, time::Duration};

use rand::{thread_rng, Rng};
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:55330");
    let udp_socket = UdpSocket::bind(addr).await.expect("cant bind to address");
    let send_to = SocketAddr::from_str("127.0.0.1:55331").expect("cant parse send_to address");

    loop {
        let mut rng = thread_rng();
        let temperature = rng.gen_range(-10.0_f32..20.0_f32);
        let mut data = [0u8; 5];
        data[0] = 4;
        data[1..5].copy_from_slice(&temperature.to_be_bytes());
        udp_socket
            .send_to(&data, send_to)
            .await
            .expect("error on sending temperature");
        tokio::time::sleep(Duration::from_millis(500)).await
    }
}
