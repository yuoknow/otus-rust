use std::time::Duration;

use crate::smart_thermometer::SmartThermometer;

pub mod smart_thermometer;

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1:55331");
    let thermo = SmartThermometer::new(addr).await.unwrap();

    for _ in 1..100 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        println!("current temperature: {} ", thermo.get_temperature())
    }
}
