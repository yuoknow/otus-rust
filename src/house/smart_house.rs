use std::collections::{HashMap, HashSet};

use crate::house::devices::device::Device;

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

pub struct Room {
    pub devices: HashMap<String, Device>,
}

impl SmartHouse {
    pub fn new(rooms: HashMap<String, Room>) -> Self {
        Self {
            name: "My house".to_string(),
            rooms,
        }
    }

    pub fn get_rooms(&self) -> HashSet<&String> {
        self.rooms.keys().collect()
    }

    pub fn get_devices(&self, room: &str) -> HashSet<&String> {
        self.rooms[room].devices.keys().collect()
    }

    pub fn create_report(&self, devices: Vec<(String, String)>) -> String {
        let mut report = "".to_string();
        report += &*format!("Report for house: {}\n", &self.name);
        for device in devices {
            report += &*format!(
                "Device {} from {} status: {:?}\n",
                device.1,
                device.0,
                &self.find_device(device.0.as_str(), device.1.as_str())
            );
        }

        report
    }

    fn find_device(&self, room: &str, device: &str) -> String {
        match &self.rooms.get(room) {
            Some(room) => match room.devices.get(device) {
                Some(device) => device.status(),
                None => "Device not found".to_string(),
            },
            None => "Room not found".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::house::devices::device::Device;
    use std::collections::{HashMap, HashSet};

    use crate::house::smart_house::{Room, SmartHouse};

    #[test]
    fn should_get_rooms() {
        let rooms = HashMap::from([
            (
                "Room1".to_string(),
                Room {
                    devices: Default::default(),
                },
            ),
            (
                "Room2".to_string(),
                Room {
                    devices: Default::default(),
                },
            ),
        ]);

        let house = SmartHouse::new(rooms);

        assert_eq!(house.get_rooms(), HashSet::from([&"Room1".to_string(), &"Room2".to_string()]));
    }

    #[test]
    fn should_get_devices() {
        let rooms = HashMap::from([(
            "Room1".to_string(),
            Room {
                devices: HashMap::from([
                    (
                        "Socket1".to_string(),
                        Device::SmartSocket(Default::default()),
                    ),
                    (
                        "Thermo".to_string(),
                        Device::SmartThermometer(Default::default()),
                    ),
                ]),
            },
        )]);

        let house = SmartHouse::new(rooms);

        assert_eq!(
            house.get_devices("Room1"),
            HashSet::from([&"Socket1".to_string(), &"Thermo".to_string()])
        );
    }

    #[test]
    fn should_find_device() {
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
            house.find_device("Room1", "Thermo"),
            "SmartThermometer { _current_temperature: 0.0 }"
        );
    }

    #[test]
    fn should_not_find_device_if_no_devices() {
        let rooms = HashMap::from([(
            "Room1".to_string(),
            Room {
                devices: HashMap::new(),
            },
        )]);
        let house = SmartHouse::new(rooms);

        assert_eq!(house.find_device("Room1", "Thermo"), "Device not found");
    }

    #[test]
    fn should_not_find_device_if_no_room() {
        let rooms = HashMap::from([(
            "Room1".to_string(),
            Room {
                devices: HashMap::new(),
            },
        )]);
        let house = SmartHouse::new(rooms);

        assert_eq!(house.find_device("Room2", "Thermo"), "Room not found");
    }
}
