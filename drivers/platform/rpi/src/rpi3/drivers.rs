use bcm::bcm2837::{serial::BCM2837_SIMPLEUART};
use novuskinc::drivers::Driver;
use crate::DEVICE_DRIVERS;
use crate::rpi3::gpio::led::Rpi3Led;
use crate::rpi3::gpio::Rpi3Gpio;
use super::console::Rpi3Console;

pub unsafe fn set_rpi3_drivers() {
    DEVICE_DRIVERS.add_driver(&Rpi3Console as &'static dyn Driver);
    DEVICE_DRIVERS.add_driver(&Rpi3Led {
        gpio: Rpi3Gpio,
    } as &'static dyn Driver);
}
