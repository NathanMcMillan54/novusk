use novuskinc::dif::get_dif_value;
use novuskinc::drivers::{names::*, get_driver};

pub unsafe fn _early_write_str(write: &str) {
    let mut driver_name = "";

    if get_dif_value("PrintingMethod").contains(SIMPLE_UART) {
        driver_name = SIMPLE_UART;
    } else if get_dif_value("PrintingMethod").contains(FRAME_BUFFER) {
        driver_name = FRAME_BUFFER;
    }

    let mut driver = get_driver(driver_name);

    if driver.is_none() { return; }
    if driver_name == SIMPLE_UART {
        for b in write.as_bytes() {
            driver.unwrap().write(*b);
        }
    } else { driver.unwrap().graphics_write_string(write, 0, 0); }
}
