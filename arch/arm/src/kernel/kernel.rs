use novuskinc::dif::get_dif_value;
use novuskinc::drivers::{names::*, get_driver};
use novuskinc::prelude::SimpleUart;

// TODO
#[no_mangle]
pub extern "C" fn arch_syscall() { }

pub unsafe fn _early_write_str(write: &str) {
    let mut driver_name = "";

    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'e'); }
    if get_driver(SIMPLE_UART).is_some() {
        driver_name = SIMPLE_UART;
    } else if get_driver(SERIAL).is_some() {
        driver_name = SERIAL;
    } else if get_driver(FRAME_BUFFER).is_some() {
        driver_name = FRAME_BUFFER;
    } else { return; }

    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'e'); }
    let mut driver = get_driver(driver_name);

    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'e'); }
    if !driver.is_some() { return; }

    if driver_name == SIMPLE_UART || driver_name == SERIAL {
        for b in write.as_bytes() {
            driver.as_mut().unwrap().write(*b);
        }
    } else { driver.as_mut().unwrap().graphics_write_string(write, 0, 0); }


    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'e'); }
}
