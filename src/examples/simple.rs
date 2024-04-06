use smarthome::{NewRoom, SmartHouse, SmartSocket, SmartThermometer};

fn main() {
    // Создаем дом для Мистера Иванова
    let house_1 = SmartHouse::new("IVANOV".to_owned(), vec![
        NewRoom {
            devices: vec![
                Box::new(SmartSocket { name: String::from("SMART_SOCKET_1") }),
                Box::new(SmartThermometer { name: String::from("SMART_THERMO_1") }),
            ]
        }
    ]);

    // Генерируем отчет для первого дома
    let report1 = house_1.create_report();
    println!("Отчет #1: {}", report1);

    // Создаем еще один дом для Мистера Петрова
    let house_2 = SmartHouse::new("PETROV".to_owned(), vec![
        NewRoom {
            devices: vec![
                Box::new(SmartSocket { name: String::from("SMART_SOCKET_2") }),
                Box::new(SmartThermometer { name: String::from("SMART_THERMO_2") }),
            ]
        }
    ]);

    // Генерируем отчет для второго дома
    let report2 = house_2.create_report();
    println!("Отчет #2: {}", report2);
}
