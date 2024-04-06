pub struct SmartSocket {
    pub name: String,
}

enum Status {
    ENABLED,
    DISABLED
}


impl SmartSocket  {
    fn get_info(&self) -> &str{
        &self.name
    }
    fn get_power_consumption() -> f64 {
        0.0
    }
    fn on(){
        todo!()
    }
    fn off(){
        todo!()
    }
}
