use smart_house::http_api::start_web_server;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    tokio::spawn(async { start_web_server().await });
    let response = reqwest::get("http://localhost:8080/house/rooms")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Get rooms response: {:?}", response);

    let client = reqwest::Client::new();

    let mut request = HashMap::new();
    request.insert("name", "new_room");

    client
        .post("http://localhost:8080/house/rooms")
        .json(&request)
        .send()
        .await
        .unwrap();

    let response = reqwest::get("http://localhost:8080/house/rooms")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Get rooms response after room add: {:?}", response);

    client
        .delete("http://localhost:8080/house/rooms")
        .json(&request)
        .send()
        .await
        .unwrap();

    let response = reqwest::get("http://localhost:8080/house/rooms")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Get rooms response after room delete: {:?}", response);

    let response = reqwest::get("http://localhost:8080/house/rooms/room1/devices")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Get devices for room1 response: {:?}", response);

    let response = reqwest::get("http://localhost:8080/house/report")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("Get report response: {:?}", response);
}
