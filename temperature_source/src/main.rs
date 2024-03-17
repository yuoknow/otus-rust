use std::net::{SocketAddr, UdpSocket};
use std::str::FromStr;
use std::thread;
use std::time::Duration;

use rand::{thread_rng, Rng};

fn main() {
    let addr = String::from("127.0.0.1:55330");
    let udp_socket = UdpSocket::bind(addr).expect("cant bind to address");
    udp_socket
        .set_read_timeout(Option::from(Duration::from_secs(1)))
        .expect("cant bind to address: read timeout");
    let send_to = SocketAddr::from_str("127.0.0.1:55331").expect("cant parse send_to address");

    loop {
        let mut rng = thread_rng();
        let temperature = rng.gen_range(-10.0_f32..20.0_f32);
        let mut data = [0u8; 5];
        data[0] = 4;
        data[1..5].copy_from_slice(&temperature.to_be_bytes());
        udp_socket
            .send_to(&data, send_to)
            .expect("error on sending temperature");
        thread::sleep(Duration::from_millis(500))
    }
}
