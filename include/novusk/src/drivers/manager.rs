use super::Driver;

extern "C" {
    /// This is a static that contains all the drivers that Novusk will use, it is defined in
    /// ``drivers/drivers.rs`` which should be included in the architecture specific kernel.
    pub static mut DRIVER_MANAGER: DeviceDriverManager;
}

/// ``DeviceDriverManager`` is used to manage main device drivers
pub struct DeviceDriverManager {
    pub drivers: Option<[&'static dyn Driver; 10]>,
}

impl DeviceDriverManager {
    pub fn new() -> Self {
        return DeviceDriverManager {
            drivers: None,
        }
    }

    /// ``add_driver`` is used to add a device driver to the manager, the "driver" argument can be
    /// any struct that implements ``Driver``
    pub fn add_driver(&mut self, driver: &'static dyn Driver) {
        if self.drivers.is_none() {
            self.drivers = Some([driver; 10])
        } else { self.drivers.unwrap().iter().map(|new_driver|{
            driver
        }); }
    }

    /// The ``get_driver`` function returns a device driver. The "name" argument is what
    /// ``driver_name`` returns from ``Driver``
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
