pub mod arm_init;
pub mod board;
pub mod cpu;
pub mod entry;
pub mod io;
pub mod irq;
pub mod kernel;
pub mod panic;
pub mod printk;
pub mod setup;

#[path = "../../../../kernel/irq.rs"]
pub mod irqs;

pub(crate) mod drivers {
    use device::arm::ArmDevice;
    use dif::Dif;
    use novuskinc::drivers::Driver;
    use novuskinc::drivers::names::*;
    use novuskinc::prelude::KernelConsoleDriver;

    #[no_mangle]
    pub static mut ARM_DEVICE: ArmDevice = ArmDevice {
        dif: Dif::new(),
        mailbox: None,
        console: None,
        serial: None,
        display: None,
        timer: None,
        keyboard: None
    };

    #[no_mangle]
    pub unsafe extern "C" fn add_driver(driver: &'static dyn Driver) {
        match driver.name() {
            CONSOLE => {
                ARM_DEVICE.console = Some(driver);
            },
            SERIAL => {
                ARM_DEVICE.serial = Some(driver);
            }
            FRAME_BUFFER => {
                ARM_DEVICE.display = Some(driver);
            },
            KEYBOARD => {
                ARM_DEVICE.keyboard = Some(driver);
            }
            _ => {},
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn get_driver(name: &'static str) -> Option<&'static dyn Driver> {
        return match name {
            CONSOLE => {
                ARM_DEVICE.console
            },
            SERIAL => {
                ARM_DEVICE.serial
            },
            FRAME_BUFFER => {
                ARM_DEVICE.display
            },
            KEYBOARD => {
                ARM_DEVICE.keyboard
            },
            _ => None,
        };
    }
}
