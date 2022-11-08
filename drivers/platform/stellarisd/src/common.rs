use hio::HioDriver;
use novuskinc::drivers::{add_driver, Driver};
use tm4c123x_hal::Peripherals;

pub unsafe fn common_early_stellaris_init() {
    if Peripherals::take().is_none() {
        panic!("Device peripherals can not be found, Novusk might be running on an unsupported device");
    }

    add_driver(&HioDriver as &'static dyn Driver);
}
