use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;

use crate::house::devices::device::Device;
use crate::house::smart_house::SmartHouseError::{DeviceNotFound, RoomNotFound};

#[derive(Debug, PartialEq, Eq)]
pub enum SmartHouseError {
    DeviceNotFound { room: String, device: String },
    RoomNotFound(String),
}

impl Error for SmartHouseError {}

impl Display for SmartHouseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceNotFound { room, device } => {
                write!(f, "Device [{}] not found in room [{}]", device, room)
            }
            RoomNotFound(room) => write!(f, "Room not found: [{}]", room),
        }
    }
}

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
                match &self.find_device(device.0.as_str(), device.1.as_str()) {
                    Ok(result) => {
                        result.to_string()
                    }
                    Err(error) => {
                        error.to_string()
                    }
                }
            );
        }

        report
    }

    fn find_device(&self, room_str: &str, device: &str) -> Result<String, SmartHouseError> {
        match self.rooms.get(room_str) {
            Some(room) => match room.devices.get(device) {
                Some(device) => Ok(device.status()),
                None => Err(DeviceNotFound {
                    room: room_str.to_string(),
                    device: device.to_string(),
                }),
            },
            None => Err(RoomNotFound(room_str.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::house::devices::device::Device;
    use std::collections::{HashMap, HashSet};

    use crate::house::smart_house::SmartHouseError::{DeviceNotFound, RoomNotFound};
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

        assert_eq!(
            house.get_rooms(),
            HashSet::from([&"Room1".to_string(), &"Room2".to_string()])
        );
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
            Ok("SmartThermometer { _current_temperature: 0.0 }".to_string())
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

        assert_eq!(
            house.find_device("Room1", "Thermo"),
            Err(DeviceNotFound {
                room: "Room1".to_string(),
                device: "Thermo".to_string()
            })
        );
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

        assert_eq!(
            house.find_device("Room2", "Thermo"),
            Err(RoomNotFound("Room2".to_string()))
        );
    }
}
