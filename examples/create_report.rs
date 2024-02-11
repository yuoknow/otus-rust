use smart_house::house::devices::device::Device;
use smart_house::house::devices::smart_socket::SmartSocket;
use smart_house::house::devices::smart_thermometer::SmartThermometer;
use smart_house::house::smart_house::SmartHouse;

fn main() {
    let socket1 = SmartSocket {
        _description: "socket1".to_string(),
        _is_enabled: false,
        _power: 10.0,
    };
    let socket2 = SmartSocket {
        _description: "socket2".to_string(),
        _is_enabled: false,
        _power: 10.0,
    };
    let thermo = SmartThermometer {
        _current_temperature: 20.0,
    };

    let mut house = SmartHouse::new();
    house.add_room("Room1".to_string()).unwrap();
    house
        .add_device(
            "Room1".to_string(),
            "Socket1".to_string(),
            Device::SmartSocket(socket1),
        )
        .unwrap();
    house
        .add_device(
            "Room1".to_string(),
            "Thermo".to_string(),
            Device::SmartThermometer(thermo),
        )
        .unwrap();

    house.add_room("Room2".to_string()).unwrap();
    house
        .add_device(
            "Room2".to_string(),
            "Socket2".to_string(),
            Device::SmartSocket(socket2),
        )
        .unwrap();

    let rooms = house.get_rooms();

    println!("Rooms: {:?}", rooms);
    for room in rooms {
        println!("Devices in room {}: {:?}", room, house.get_devices(room));
    }

    let report1 = house.create_report(Vec::from([
        ("Room1".to_string(), "Socket1".to_string()),
        ("Room2".to_string(), "Socket1".to_string()),
        ("Room3".to_string(), "Socket1".to_string()),
        ("Room1".to_string(), "Thermo".to_string()),
        ("Room2".to_string(), "Socket2".to_string()),
    ]));

    println!("Report #1: {report1}");
}
