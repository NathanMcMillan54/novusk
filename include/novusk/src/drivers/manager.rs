use super::Driver;

pub struct DeviceDriverManager {
    pub drivers: Option<[&'static dyn Driver; 10]>,
}

impl DeviceDriverManager {
    pub fn new() -> Self {
        return DeviceDriverManager {
            drivers: None,
        }
    }

    pub fn add_driver(&mut self, driver: &'static dyn Driver) {
        if self.drivers.is_none() {
            self.drivers = Some([driver; 10])
        } else { self.drivers.unwrap().iter().map(|new_driver|{
            driver
        }); }
    }

    pub fn get_driver(&mut self, name: &'static str) -> Option<&'static dyn Driver> {
        if self.drivers.is_none() {
            return None;
        } else {
            for d in 0..self.drivers.unwrap().len() {
                if name == self.drivers.unwrap()[d].name() {
                    return Some(self.drivers.unwrap()[d]);
                }
            }
        }

        return None;
    }
}
