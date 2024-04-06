pub mod smart_thermometer;
pub mod smart_socket;
pub use smart_socket::SmartSocket;
pub use smart_thermometer::SmartThermometer;



pub trait Device {
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
