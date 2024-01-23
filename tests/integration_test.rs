use std::collections::HashMap;

use smart_house::house::devices::device::Device;
use smart_house::house::smart_house::{Room, SmartHouse};

#[test]
fn should_create_report() {
    let expected = "Report for house: My house\nDevice Thermo from Room1 status: \"SmartThermometer { _current_temperature: 0.0 }\"\n";
    let rooms = HashMap::from([(
        "Room1".to_string(),
        Room {
            devices: HashMap::from([(
                "Thermo".to_string(),
                Device::SmartThermometer(Default::default()),
            )]),
        },
    )]);
    let house = SmartHouse::new(rooms);

    assert_eq!(
        house.create_report(Vec::from([("Room1".to_string(), "Thermo".to_string())])),
        expected
    );
}
