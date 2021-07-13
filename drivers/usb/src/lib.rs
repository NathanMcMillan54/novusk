#![no_std]

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DriverNames {
    Nrf,
    None,
}

pub static mut USB_DRIVER: DriverNames = DriverNames::None;

#[no_mangle]
pub unsafe extern "C" fn set_usb_driver(driver: DriverNames) {
    USB_DRIVER = driver;
}



