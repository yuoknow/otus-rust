use smart_socket_client::SmartSocket;

fn main() {
    let mut smart_socket = SmartSocket::default();
    smart_socket.enable();

    println!("{:?}", smart_socket);

    smart_socket.disable();

    println!("{:?}", smart_socket);
}
