use std::thread;
use std::time::Duration;

use crate::smart_thermometer::SmartThermometer;

pub mod smart_thermometer;

fn main() {
    let addr = String::from("127.0.0.1:55331");
    let thermo = SmartThermometer::new(addr).unwrap();

    for _ in 1..100 {
        thread::sleep(Duration::from_secs(1));
        println!("current temperature: {} ", thermo.get_temperature())
    }
}
