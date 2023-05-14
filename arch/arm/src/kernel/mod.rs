pub mod kernel;

// Driver handler for when ``allocator`` feature is selected
#[cfg(feature = "allocator")]
pub(crate) mod drivers {
    use device::arm::ArmDevice;
    use dif::Dif;
    use novuskinc::drivers::{manager::DeviceDriverManager, Driver};

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

// Driver handler for when the ``allocator`` feature isn't selected
#[cfg(not(feature = "allocator"))]
pub(crate) mod drivers {
    use core::borrow::{Borrow, BorrowMut};
    use core::cell::Cell;
    use core::ops::{Deref, DerefMut};
    use cortex_a::asm::ret;
    use device::arm::ArmDevice;
    use dif::Dif;
    use novuskinc::drivers::Driver;
    use novuskinc::drivers::names::*;
    use novuskinc::prelude::KernelConsoleDriver;

    #[no_mangle]
    pub static mut ARM_DEVICE: spin::Mutex<ArmDevice> = spin::Mutex::new(ArmDevice {
        dif: Dif::new(),
        mailbox: None,
        console: None,
        serial: None,
        simple_uart: None,
        display: None,
        timer: None,
        keyboard: None
    });

    #[no_mangle]
    pub unsafe extern "C" fn add_driver(driver: &'static mut dyn Driver) {
        match driver.name() {
            CONSOLE => {
                ARM_DEVICE.lock().console = Some(driver);
            },
            SERIAL => {
                ARM_DEVICE.lock().serial = Some(driver);
            }
            SIMPLE_UART => {
                ARM_DEVICE.lock().simple_uart = Some(driver);
            }
            FRAME_BUFFER => {
                ARM_DEVICE.lock().display = Some(driver);
            },
            KEYBOARD => {
                ARM_DEVICE.lock().keyboard = Some(driver);
            }
            _ => {},
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_driver(name: &'static str) -> &Option<&'static mut dyn Driver> {
        let mut device = ARM_DEVICE.get_mut();

        return match name {
            CONSOLE => {
                let console = &device.console;
                console
            },
            SERIAL => {
                let serial = &device.serial;
                serial
            },
            SIMPLE_UART => {
                let simple_uart = &device.simple_uart;
                simple_uart
            },
            FRAME_BUFFER => {
                let frame_buffer = &device.display;
                frame_buffer
            },
            KEYBOARD => {
                let keyboard = &device.keyboard;
                keyboard
            }
            _ => { &None }
        };
    }
}
