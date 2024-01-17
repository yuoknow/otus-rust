use std::collections::HashMap;

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

    let room1 = Room {
        devices: HashMap::from([
            ("Socket1".to_string(), Device::SmartSocket(socket1)),
            ("Thermo".to_string(), Device::SmartThermometer(thermo)),
        ]),
    };

    let room2 = Room {
        devices: HashMap::from([("Socket2".to_string(), Device::SmartSocket(socket2))]),
    };

    let rooms = HashMap::from([("Room1".to_string(), room1), ("Room2".to_string(), room2)]);

    let house = SmartHouse::new(rooms);
    let rooms = house.get_rooms();

    println!("Rooms: {:?}", rooms);
    for room in rooms {
        println!("Devices in room {}: {:?}", room, house.devices(room));
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

enum Device {
    SmartSocket(SmartSocket),
    SmartThermometer(SmartThermometer),
}

impl Device {
    fn status(&self) -> String {
        match self {
            Device::SmartThermometer(state) => format!("{:?}", state),
            Device::SmartSocket(state) => format!("{:?}", state),
        }
    }
}

struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

struct Room {
    devices: HashMap<String, Device>,
}

impl SmartHouse {
    fn new(rooms: HashMap<String, Room>) -> Self {
        Self {
            name: "My house".to_string(),
            rooms,
        }
    }

    fn get_rooms(&self) -> Vec<&String> {
        self.rooms.keys().collect()
    }

    fn devices(&self, room: &str) -> Vec<&String> {
        self.rooms[room].devices.keys().collect()
    }

    fn create_report(&self, devices: Vec<(String, String)>) -> String {
        let mut report = "".to_string();
        report += &*format!("Report for house: {}\n", &self.name);
        for device in devices {
            report += &*format!(
                "Device {} from {} status: {:?}\n",
                device.1,
                device.0,
                match &self.rooms.get(&device.0) {
                    Some(room) => match room.devices.get(&device.1) {
                        Some(device) => device.status(),
                        None => "Device not found".to_string(),
                    },
                    None => "Room not found".to_string(),
                }
            );
        }

        report
    }
}

#[derive(Debug)]
struct SmartSocket {
    _description: String,
    _is_enabled: bool,
    _power: f64,
}

#[derive(Debug)]
struct SmartThermometer {
    _current_temperature: f64,
}

impl SmartSocket {
    fn _enable(&mut self) {
        self._is_enabled = true
    }

    fn _disable(&mut self) {
        self._is_enabled = false
    }

    fn _get_description(self) -> String {
        self._description
    }

    fn _is_enabled(&self) -> bool {
        self._is_enabled
    }

    fn _get_power(&self) -> f64 {
        self._power
    }
}

impl SmartThermometer {
    fn _get_current_temperature(&self) -> f64 {
        self._current_temperature
    }
}
