 
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

    fn get_devices(&self) -> Vec<T>{
        &self.devices
    }

}

struct SmartHouse {
    name: String,
    rooms: Vec<Room<dyn Device>>
}

impl SmartHouse {
    fn new(owner_name: String, mut unupdated_rooms: Vec<Room<dyn Device>>) -> Self {

        let house_name = format!("{}_HOUSE", owner_name);

        let rooms: Vec<Room> = unupdated_rooms.into_iter().enumerate().map(|(index, mut room)| {
            room.name = format!("{}_{}_ROOM", house_name, index + 1);
            room
        }).collect();

        SmartHouse {
            name: house_name,
            rooms
        }
    }
    
    fn get_rooms(&self) -> &Vec<Room<dyn Device>> {
        &self.rooms
    }


    fn create_report(&self) -> String {
       
        let mut result = String::new(); 

        for room in &self.rooms {
            result.push_str(&format!("{} has devices: ", room.name));
            let devices_names: Vec<&str> = room.devices.iter().map(|device| &device.name[..]).collect();
            result.push_str(&devices_names.join(", "));
            result.push_str("\n");
        }
        result 
    }
}


fn main() {

    let house_1 = SmartHouse::new("KHAMBELOV".to_owned(), vec![
        SmartSocket{ name: String::from("SMART_SOCKET_1") } ,
        SmartSocket{  name: String::from("SMART_SOCKET_2") },
        SmartThermometer{  name: String::from("SMART_THERMO_1") }
    ]);

    let house_2 = SmartHouse::new("IVANOV".to_owned(), vec![
        SmartSocket{ name: String::from("SMART_SOCKET_1") } ,                
        SmartThermometer{  name: String::from("SMART_THERMO_1") }

    ]);

    let report1= house_1.create_report();

    println!("Report #1: {report1}");

    let report2 = house_2.create_report();

    println!("Report #1: {report2}");

}
    // Инициализация устройств
    // let socket1 =  SmartSocket { name: "SMART_SOCKET_1".to_string(), room: '1'.to_string()};
    // let socket2 = SmartSocket { name: "SMART_SOCKET_2".to_string(), room: '2'.to_string() };
    // let thermo = SmartThermometer { name: "SMART_THERMOMETER_1".to_string(), room: '2'.to_string() };

    // Инициализация дома
    // // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    // let info_provider_1: OwningDeviceInfoProvider = OwningDeviceInfoProvider {
    //     socket: socket1,
    // };
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report1: String = house.create_report(&info_provider_1);

    // // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2: BorrowingDeviceInfoProvider<'_> = BorrowingDeviceInfoProvider {
    //     socket: &socket2,
    //     thermo: &thermo,
    // };

    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    // let report2 = house.create_report(&info_provider_2);

    // // Выводим отчёты на экран:
    // println!("Report #1: {report1}");
    // println!("Report #2: {report2}");
// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

    // fn create_report<T: DeviceInfoProvider>(
    //     &self, 
    //     info_provider: &T
    //     /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    // ) -> String {
    //     let mut result = String::new(); 
    //     for (name, room) in info_provider.get_state() {
    //         result.push_str(&name.to_string());
    //         result.push_str(" - ");
    //         result.push_str(&room.to_string()); 
    //         result.push_str("\n"); 
    //     }
    //     result 
    //     // todo!("перебор комнат и устройств в них для составления отчёта")
    // }

// ***** Пример использования библиотеки умный дом:

// Пользовательские устройства:


// impl Device for SmartSocket  {
//     fn get_name(&self) -> &str {
//         self.get_name()
//     }
// }

// // Пользовательские поставщики информации об устройствах.
// // Могут как хранить устройства, так и заимствывать.
// struct OwningDeviceInfoProvider {
//     socket: SmartSocket,
// }
// struct BorrowingDeviceInfoProvider<'a> {
//     socket: &'a SmartSocket,
//     thermo: &'a SmartThermometer,
// }

// trait DeviceInfoProvider {
//     fn get_state(&self) -> Vec<(String, String)>;
// }

// impl DeviceInfoProvider for OwningDeviceInfoProvider {
//     fn get_state(&self) -> Vec<(String, String)> {
//         vec![(self.socket.name.clone(), self.socket.room.clone())]
//     }
// }

// impl DeviceInfoProvider for BorrowingDeviceInfoProvider<'_> {
//     fn get_state(&self) -> Vec<(String, String)> {
//         vec![(self.socket.name.clone(), self.socket.room.clone()), (self.thermo.name.clone(), self.thermo.room.clone())]
//     }
// }    // fn get_room_names(&self) -> &[&str] {
    //     let room_names: Vec<&str> = self.rooms.iter().map(|room| &room.name[..]).collect();
    //     &room_names[..]
    // }

    // fn get_devices(&self, room: &str) -> [&str; 3] {
    //     // Размер возвращаемого массива можно выбрать самостоятельно
    //     todo!("список устройств в комнате `room`")
    // }
