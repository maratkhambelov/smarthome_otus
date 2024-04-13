use smarthome::{SmartHouse, SmartSocket, SmartThermometer};

fn main() {
    let house_1 = SmartHouse::new(
        "IVANOV".to_owned(),
        vec![vec![
            Box::new(SmartSocket {
                name: String::from("SMART_SOCKET_1"),
            }),
            Box::new(SmartThermometer {
                name: String::from("SMART_THERMO_1"),
            }),
        ]],
    );

    let report1 = house_1.create_report();
    println!("Отчет #1: {}", report1);

    let house_2 = SmartHouse::new(
        "PETROV".to_owned(),
        vec![vec![
            Box::new(SmartSocket {
                name: String::from("SMART_SOCKET_2"),
            }),
            Box::new(SmartThermometer {
                name: String::from("SMART_THERMO_2"),
            }),
        ]],
    );

    let report2 = house_2.create_report();
    println!("Отчет #2: {}", report2);
}
