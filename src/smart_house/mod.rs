use crate::devices::Device;

struct Room<T: Device> {
    name: String,
    pub devices: Vec<T>
}


impl<T: Device> Room<T> {
    fn new(name: String, devices: Vec<T>) -> Self {
        Room {
            name,
            devices
        }
    } 

    pub fn add_device(&mut self, device: T){
        self.devices.push(device)
    }

    pub fn get_devices(&self) -> &Vec<T>{
        &self.devices
    }
}



pub struct NewRoom<T: Device> {
    pub devices: Vec<T>
}

pub struct SmartHouse {
    name: String,
    rooms: Vec<Room<Box<dyn Device>>>
}
impl SmartHouse {
    pub fn new(owner_name: String, new_rooms: Vec<NewRoom<Box<dyn Device>>>) -> Self {

        let house_name = format!("{}_HOUSE", owner_name);

        let rooms: Vec<Room<_>> = new_rooms.into_iter().enumerate().map(|(index,new_room)| {
            Room {
                name: format!("{}_{}_ROOM", house_name, index + 1),
                devices: new_room.devices
            }
        }).collect();

        SmartHouse {
            name: house_name,
            rooms
        }
    }
    
    pub fn get_rooms(&self) -> &Vec<Room<Box<dyn Device>>> {
        &self.rooms
    }


    pub fn create_report(&self) -> String {
       
        let mut result = String::new(); 

        for room in &self.rooms {
            result.push_str(&format!("{} has devices: ", room.name));
            let devices_names: Vec<&str> = room.devices.iter().map(|device| device.get_name()).collect();
            result.push_str(&devices_names.join(", "));
            result.push_str("\n");
        }
        result 
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Mock Device implementation for testing
    #[derive(Debug, Clone)]
    struct MockDevice {
        pub name: String,
    }

    impl Device for MockDevice {
        fn get_name(&self) -> &str {
            &self.name
        }
    }

    impl MockDevice {
        fn new(name: &str) -> Self {
            MockDevice {
                name: name.to_string(),
            }
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
    fn test_get_devices() {
        let room: Room<MockDevice> =
            Room::new("Bedroom".to_string(), vec![MockDevice::new("Test Device")]);
        let devices = room.get_devices();
        assert_eq!(devices.len(), 1);
        assert_eq!(devices[0].get_name(), "Test Device");
    }

    #[test]
    fn test_create_report() {
        let owner_name = "KHAMBELOV".to_string();
        let device_a = "DEVICE_A".to_string();  
        let device_b = "DEVICE_B".to_string(); 

        let house = SmartHouse::new(
            owner_name.clone(),
            vec![NewRoom {
                devices: vec![
                    Box::new(MockDevice { name: String::from(device_a.clone()) }),
                    Box::new(MockDevice { name: String::from(device_b.clone()) }),
                ],
            }],
        );
        let report = house.create_report();

        let expected_house_name = "KHAMBELOV_HOUSE_1_ROOM".to_string();
        assert!(report.contains(&format!(
            "{} has devices: {}, {}",
            expected_house_name, device_a, device_b
        )));
    }
}
