  
struct SmartSocket {
    name: String
}
struct SmartThermometer {
    name: String
}


trait Device {
    fn get_name(&self) -> &str;
}

impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Device for SmartThermometer {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Device for Box<dyn Device> {
    fn get_name(&self) -> &str {         
        (**self).get_name()
    }
}

struct Room<T: Device> {
    name: String,
    devices: Vec<T>
}


impl<T: Device> Room<T> {
    fn new(name: String, devices: Vec<T>) -> Self {
        Room {
            name,
            devices
        }
    } 

    fn add_device(&mut self, device: T){
        self.devices.push(device)
    }

    fn get_devices(&self) -> &Vec<T>{
        &self.devices
    }
}

struct SmartHouse {
    name: String,
    rooms: Vec<Room<Box<dyn Device>>>
}

struct NewRoom<T: Device> {
    devices: Vec<T>
}

impl SmartHouse {
    fn new(owner_name: String, new_rooms: Vec<NewRoom<Box<dyn Device>>>) -> Self {

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
    
    fn get_rooms(&self) -> &Vec<Room<Box<dyn Device>>> {
        &self.rooms
    }


    fn create_report(&self) -> String {
       
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


fn main() {



    let house_1 = SmartHouse::new("KHAMBELOV".to_owned(), vec![NewRoom {
        devices: vec![
            Box::new(SmartSocket { name: String::from("SMART_SOCKET_1") }),
            Box::new(SmartSocket { name: String::from("SMART_SOCKET_2") }),
            Box::new(SmartThermometer {  name: String::from("SMART_THERMO_1") })
        ]
    }]);

    let report1= house_1.create_report();

    println!("Report #1: {report1}");

    let house_2 = SmartHouse::new("IVANOV".to_owned(), 
      
    vec![NewRoom {
      devices: vec![
          Box::new(SmartSocket { name: String::from("SMART_SOCKET_1") }),
          Box::new(SmartThermometer {  name: String::from("SMART_THERMO_1") })
      ]
  }]);
    
  let report2 = house_2.create_report();

  println!("Report #1: {report2}");

}
    