pub mod kernel;

pub(crate) mod drivers {
    use device::arm::ArmDevice;
    use dif::Dif;
    use novuskinc::drivers::{manager::DeviceDriverManager, Driver};
    /*use novuskinc::drivers::names::*;
    use novuskinc::prelude::KernelConsoleDriver;

    #[no_mangle]
    pub static mut ARM_DEVICE: ArmDevice = ArmDevice {
        dif: Dif::new(),
        mailbox: None,
        console: None,
        serial: None,
        simple_uart: None,
        display: None,
        timer: None,
        keyboard: None
    };*/

    #[cfg(feature = "allocator")]
    #[no_mangle]
    pub static mut DEVICE_DRIVERS: DeviceDriverManager = DeviceDriverManager {
        drivers: alloc::vec![],
    };

    #[no_mangle]
    pub unsafe extern "C" fn add_driver(driver: &'static mut dyn Driver) {
        DEVICE_DRIVERS.add_driver(driver);
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_driver(name: &'static str) -> Option<&'static mut dyn Driver> {
        DEVICE_DRIVERS.get_driver(name)
    }
}
