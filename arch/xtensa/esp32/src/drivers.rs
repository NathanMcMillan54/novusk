use net::{set_net};

pub unsafe fn esp32_drivers_init() {
    set_net("ESP-WROOM-32");
}
