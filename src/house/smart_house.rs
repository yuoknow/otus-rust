use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::house::devices::device::Device;
use crate::house::smart_house::SmartHouseError::{
    DeviceAlreadyExists, DeviceNotFound, RoomAlreadyExists, RoomNotFound,
};

#[derive(Error, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SmartHouseError {
    #[error("Device [{device}] not found in room [{room}]")]
    DeviceNotFound { room: String, device: String },
    #[error("Room not found: [{0}]")]
    RoomNotFound(String),
    #[error("Room already exists: [{0}]")]
    RoomAlreadyExists(String),
    #[error("Device [{device}] already exists in room [{room}]")]
    DeviceAlreadyExists { room: String, device: String },
}

pub struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>,
}

impl Default for SmartHouse {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Room {
    pub devices: HashMap<String, Device>,
}

impl Default for Room {
    fn default() -> Self {
        Self::new()
    }
}

impl Room {
    pub fn new() -> Self {
        Self {
            devices: HashMap::new(),
        }
    }
}

impl SmartHouse {
    pub fn new() -> Self {
        Self {
            name: "My house".to_string(),
            rooms: HashMap::new(),
        }
    }

    pub fn of(rooms: HashMap<String, Room>) -> Self {
        Self {
            name: "My house".to_string(),
            rooms,
        }
    }

    pub fn add_room(&mut self, room: String) -> Result<(), SmartHouseError> {
        match self.rooms.entry(room) {
            Entry::Occupied(room) => Err(RoomAlreadyExists(room.key().to_string())),
            Entry::Vacant(vacant) => {
                vacant.insert(Room::new());
                Ok(())
            }
        }
    }

    pub fn remove_room(&mut self, room: String) -> Result<(), SmartHouseError> {
        match self.rooms.entry(room) {
            Entry::Occupied(room) => {
                room.remove();
                Ok(())
            }
            Entry::Vacant(vacant) => Err(RoomNotFound(vacant.key().to_string())),
        }
    }

    pub fn add_device(
        &mut self,
        room: String,
        device_name: String,
        device: Device,
    ) -> Result<(), SmartHouseError> {
        match self.rooms.get_mut(&room) {
            None => Err(RoomNotFound(room)),
            Some(some) => match some.devices.entry(device_name) {
                Entry::Occupied(device) => Err(DeviceAlreadyExists {
                    device: device.key().to_string(),
                    room,
                }),
                Entry::Vacant(vacant) => {
                    vacant.insert(device);
                    Ok(())
                }
            },
        }
    }

    pub fn remove_device(&mut self, room: String, device: String) -> Result<(), SmartHouseError> {
        match self.rooms.get_mut(&room) {
            None => Err(RoomNotFound(room)),
            Some(some) => match some.devices.entry(device) {
                Entry::Occupied(device) => {
                    device.remove();
                    Ok(())
                }
                Entry::Vacant(vacant) => Err(DeviceNotFound {
                    room,
                    device: vacant.key().to_string(),
                }),
            },
        }
    }

    pub fn get_rooms(&self) -> HashSet<&String> {
        self.rooms.keys().collect()
    }

    pub fn get_devices(&self, room: &str) -> HashSet<&String> {
        self.rooms[room].devices.keys().collect()
    }

    pub fn get_all_devices(&self) -> Vec<(String, String)> {
        let mut devices = Vec::new();
        for room in &self.rooms {
            for device in &room.1.devices {
                devices.push((room.0.to_string(), device.0.to_string()))
            }
        }
        devices
    }

    pub fn get_device(&self, room: &str, device: &str) -> Device {
        self.rooms[room].devices.get(device).unwrap().clone()
    }

    pub fn create_all_devices_report(&self) -> String {
        self.create_report(self.get_all_devices())
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
    use std::collections::{HashMap, HashSet};

    use crate::house::devices::device::Device;
    use crate::house::smart_house::SmartHouseError::{
        DeviceAlreadyExists, DeviceNotFound, RoomAlreadyExists, RoomNotFound,
    };
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

        let house = SmartHouse::of(rooms);

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

        let house = SmartHouse::of(rooms);

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
        let house = SmartHouse::of(rooms);

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
        let house = SmartHouse::of(rooms);

        assert_eq!(
            house.find_device("Room1", "Thermo"),
            Err(DeviceNotFound {
                room: "Room1".to_string(),
                device: "Thermo".to_string(),
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
        let house = SmartHouse::of(rooms);

        assert_eq!(
            house.find_device("Room2", "Thermo"),
            Err(RoomNotFound("Room2".to_string()))
        );
    }

    #[test]
    fn should_add_new_room() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();

        assert_eq!(house.get_rooms(), HashSet::from([&"Room1".to_string()]));
    }

    #[test]
    fn should_not_add_the_same_room() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();

        assert_eq!(
            house.add_room("Room1".to_string()),
            Err(RoomAlreadyExists("Room1".to_string()))
        );
    }

    #[test]
    fn should_remove_room() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();
        house.remove_room("Room1".to_string()).unwrap();

        assert_eq!(house.get_rooms(), HashSet::from([]));
    }

    #[test]
    fn should_not_remove_room_if_no_room() {
        let mut house = SmartHouse::new();

        assert_eq!(
            house.remove_room("Room1".to_string()),
            Err(RoomNotFound("Room1".to_string()))
        );
    }

    #[test]
    fn should_add_device_to_room() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();
        house
            .add_device(
                "Room1".to_string(),
                "Thermo".to_string(),
                Device::SmartThermometer(Default::default()),
            )
            .unwrap();

        assert_eq!(
            house.get_devices("Room1"),
            HashSet::from([&"Thermo".to_string()])
        );
    }

    #[test]
    fn should_not_add_the_same_device_to_room() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();
        house
            .add_device(
                "Room1".to_string(),
                "Thermo".to_string(),
                Device::SmartThermometer(Default::default()),
            )
            .unwrap();

        assert_eq!(
            house.add_device(
                "Room1".to_string(),
                "Thermo".to_string(),
                Device::SmartThermometer(Default::default()),
            ),
            Err(DeviceAlreadyExists {
                room: "Room1".to_string(),
                device: "Thermo".to_string(),
            })
        );
    }

    #[test]
    fn should_remove_device() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();
        house
            .add_device(
                "Room1".to_string(),
                "Thermo".to_string(),
                Device::SmartThermometer(Default::default()),
            )
            .unwrap();
        house
            .remove_device("Room1".to_string(), "Thermo".to_string())
            .unwrap();

        assert_eq!(house.get_devices("Room1"), HashSet::from([]));
    }

    #[test]
    fn should_not_remove_device_if_no_device() {
        let mut house = SmartHouse::new();
        house.add_room("Room1".to_string()).unwrap();

        assert_eq!(
            house.remove_device("Room1".to_string(), "Thermo".to_string()),
            Err(DeviceNotFound {
                device: "Thermo".to_string(),
                room: "Room1".to_string(),
            })
        );
    }
}
