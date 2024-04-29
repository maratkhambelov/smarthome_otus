use crate::devices::Device;
#[derive(Debug)]
pub struct Room<T: Device> {
    name: String,
    pub devices: Vec<T>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GetDataError {
    RoomNotFound,
    DeviceNotFound,
}

impl<T: Device> Room<T> {
    fn new(name: String, devices: Vec<T>) -> Self {
        Room { name, devices }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    pub fn get_devices(&self) -> &Vec<T> {
        &self.devices
    }

    pub fn add_device(&mut self, device: T) {
        self.devices.push(device)
    }

    pub fn remove_device(&mut self, name: String) -> Option<T> {
        if let Some(index) = self
            .devices
            .iter()
            .position(|device| device.get_name() == name)
        {
            Some(self.devices.remove(index))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct SmartHouse {
    name: String,
    rooms: Vec<Room<Box<dyn Device>>>,
}

impl SmartHouse {
    pub fn new(owner_name: String, devices_in_rooms: Vec<Vec<Box<dyn Device>>>) -> Self {
        let house_name = format!("{}_HOUSE", owner_name);

        let rooms: Vec<Room<_>> = devices_in_rooms
            .into_iter()
            .enumerate()
            .map(|(index, devices)| Room {
                name: format!("{}_{}_ROOM", house_name, index + 1),
                devices,
            })
            .collect();

        SmartHouse {
            name: house_name,
            rooms,
        }
    }

    pub fn get_rooms(&self) -> &Vec<Room<Box<dyn Device>>> {
        &self.rooms
    }

    pub fn get_room(&self, room_name: String) -> Option<&Room<Box<dyn Device>>> {
        self.rooms.iter().find(|room| room.name == room_name)
    }

    pub fn add_room(&mut self) {
        let house_name = self.name.clone();
        self.rooms.push(Room {
            name: format!("{}_{}_ROOM", house_name, self.rooms.len() + 1),
            devices: vec![],
        })
    }

    pub fn remove_room(&mut self, name: String) -> Option<Room<Box<dyn Device>>> {
        if let Some(index) = self.rooms.iter().position(|device| device.name == name) {
            Some(self.rooms.remove(index))
        } else {
            None
        }
    }

    pub fn get_device(
        &self,
        room_name: String,
        device_name: String,
    ) -> Result<&Box<dyn Device>, GetDataError> {
        let room = match self.get_room(room_name) {
            Some(room) => room,
            None => return Err(GetDataError::RoomNotFound),
        };

        let device = match room
            .devices
            .iter()
            .find(|device| device.get_name() == device_name)
        {
            Some(device) => device,
            None => return Err(GetDataError::DeviceNotFound),
        };

        Ok(device)
    }

    pub fn create_report(&self) -> String {
        let mut result = String::new();

        for room in &self.rooms {
            result.push_str(&format!("{} has devices: ", room.name));
            let devices_names: Vec<&str> = room
                .devices
                .iter()
                .map(|device| device.get_name())
                .collect();
            result.push_str(&devices_names.join(", "));
            result.push_str("\n");
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const OWNER_NAME: &str = "KHAMBELOV";

    #[derive(Debug, Clone, PartialEq)]
    struct MockDevice {
        pub name: String,
    }

    impl MockDevice {
        fn new(name: &str) -> Self {
            MockDevice {
                name: name.to_string(),
            }
        }
    }

    impl Device for MockDevice {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_add_device() {
        let mut room: Room<MockDevice> = Room::new("Living Room".to_string(), vec![]);
        let device = MockDevice::new("Test Device");
        room.add_device(device.clone());
        assert_eq!(room.devices.len(), 1);
        assert_eq!(room.devices[0].get_name(), device.get_name());
    }

    #[test]
    fn test_delete_device() {
        let mut room = Room::new("Living Room".to_string(), vec![]);
        room.add_device(MockDevice::new("Test Device"));
        let first_device = room
            .get_devices()
            .first()
            .iter()
            .map(|device| device.get_name())
            .next();

        if let Some(name) = first_device {
            let mut amount = room.get_devices().len();
            let mut expected_amount = 1;
            assert_eq!(expected_amount, amount);
            room.remove_device(name.to_string());
            amount = room.get_devices().len();
            expected_amount = 0;
            assert_eq!(expected_amount, amount);
        } else {
            panic!("There is not device in room");
        }
    }
    #[test]
    fn test_get_devices() {
        let room: Room<MockDevice> =
            Room::new("Bedroom".to_string(), vec![MockDevice::new("Test Device")]);
        let devices = room.get_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].get_name(), "Test Device");
    }

    #[test]
    fn test_create_report() {
        let owner_name = OWNER_NAME.to_string();
        let device_a = "DEVICE_A".to_string();
        let device_b = "DEVICE_B".to_string();
        let house = SmartHouse::new(
            owner_name.clone(),
            vec![vec![
                Box::new(MockDevice {
                    name: String::from(device_a.clone()),
                }),
                Box::new(MockDevice {
                    name: String::from(device_b.clone()),
                }),
            ]],
        );

        let report = house.create_report();
        let amount_rooms = 1;

        let expected_room_name = format!("{}_HOUSE_{}_ROOM", OWNER_NAME, amount_rooms);

        assert!(report.contains(&format!(
            "{} has devices: {}, {}",
            expected_room_name, device_a, device_b
        )));
    }

    #[test]
    fn test_get_room_existing() {
        let amount_rooms = 1;
        let expected_room_name = format!("{}_HOUSE_{}_ROOM", OWNER_NAME, amount_rooms).to_string();
        let device_name = "Test_Device".to_string();
        let device = Box::new(MockDevice {
            name: device_name.clone(),
        });

        let house = SmartHouse::new(OWNER_NAME.to_string(), vec![vec![device.clone()]]);
        println!("House: {:?}", house);
        let room = house.get_room(expected_room_name.clone()).unwrap();
        println!("Room: {:?}", room);
        assert_eq!(room.name, expected_room_name);
        assert_eq!(room.devices.len(), 1);
        assert_eq!(room.devices[0].get_name(), device_name);
    }

    #[test]
    fn test_get_device_existing() {
        let amount_rooms = 1;
        let expected_room_name = format!("{}_HOUSE_{}_ROOM", OWNER_NAME, amount_rooms).to_string();
        let device_name = "Test_Device".to_string();
        let device = Box::new(MockDevice {
            name: device_name.clone(),
        });

        let house = SmartHouse::new(OWNER_NAME.to_string(), vec![vec![device.clone()]]);
        let retrieved_device = house
            .get_device(expected_room_name.clone(), device_name.clone())
            .unwrap();
        assert_eq!(retrieved_device.get_name(), device_name);
    }

    #[test]
    fn test_get_room_not_found() {
        let room_name = "Nonexistent_ROOM".to_string();
        let house = SmartHouse::new("TestOwner".to_string(), vec![]);
        let result = house.get_room(room_name);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_device_room_not_found() {
        let house = SmartHouse::new("TestOwner".to_string(), vec![]);
        let room_name = "Nonexistent_ROOM".to_string();
        let device_name = "Test_Device".to_string();
        let result = house.get_device(room_name, device_name);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(GetDataError::RoomNotFound));
    }

    #[test]
    fn test_get_device_device_not_found() {
        let device_name = "Nonexistent_Device".to_string();
        let amount_rooms = 1;
        let expected_room_name = format!("{}_HOUSE_{}_ROOM", OWNER_NAME, amount_rooms).to_string();
        let device = Box::new(MockDevice {
            name: "Test_Device".to_string(),
        });

        let house = SmartHouse::new(OWNER_NAME.to_string(), vec![vec![device]]);
        let result = house.get_device(expected_room_name, device_name);
        assert!(result.is_err());
        assert_eq!(result.err(), Some(GetDataError::DeviceNotFound));
    }
    #[test]
    fn test_add_room() {
        let mut house: SmartHouse = SmartHouse::new(OWNER_NAME.to_string(), vec![]);
        let initial_amount_rooms: usize = house.get_rooms().len();
        let mut expected_amount_rooms = 0;
        assert_eq!(expected_amount_rooms, initial_amount_rooms);
        house.add_room();
        let amount_rooms = house.get_rooms().len();
        expected_amount_rooms = 1;
        assert_eq!(expected_amount_rooms, amount_rooms);
    }
    #[test]
    fn test_delete_room() {
        let device_a = "DEVICE_A".to_string();
        let mut house = SmartHouse::new(
            OWNER_NAME.to_string(),
            vec![
                vec![Box::new(MockDevice {
                    name: String::from(device_a.clone()),
                })],
                vec![],
            ],
        );

        let first_room_name = house
            .get_rooms()
            .first()
            .iter()
            .map(|room| room.get_name())
            .next();
        if let Some(name) = first_room_name {
            let mut amount_rooms: usize = house.get_rooms().len();
            let mut expected_amount_rooms = 2;
            assert_eq!(expected_amount_rooms, amount_rooms);
            house.remove_room(name);
            amount_rooms = house.get_rooms().len();
            expected_amount_rooms = 1;
            assert_eq!(expected_amount_rooms, amount_rooms);
        } else {
            panic!("There is no rooms in the house");
        }
    }
}
