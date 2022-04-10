use super::Driver;

pub struct DeviceDriverManager {
    pub drivers: Option<[&'static (dyn Driver + Sync); 10]>,
}

impl DeviceDriverManager {
    pub fn new() -> Self {
        return DeviceDriverManager {
            drivers: None,
        }
    }
}
