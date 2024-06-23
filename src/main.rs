use smart_house::http_api::start_web_server;

#[tokio::main]
async fn main() {
    start_web_server().await
}
