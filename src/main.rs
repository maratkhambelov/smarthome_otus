use smarthome::{NewRoom, SmartHouse, SmartSocket, SmartThermometer};


fn main() {

    let house_1 = SmartHouse::new("KHAMBELOV".to_owned(), vec![NewRoom {
        devices: vec![
            Box::new(SmartSocket { name: String::from("SMART_SOCKET_1") }),
            Box::new(SmartSocket { name: String::from("SMART_SOCKET_2") }),
            Box::new(SmartThermometer { name: String::from("SMART_THERMO_1") })
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
    