#![no_std]

#[macro_use] extern crate kerror;

// Net drivers
extern crate esp32net;

pub static mut NET: &'static str = "";
pub static mut INIT: bool = false;

#[no_mangle]
pub unsafe extern "C" fn net_init() {
    if NET == "" && INIT == false {
        kerror!("Net never set or initialized, will not initialize net drivers");
        NET = "none";
    } else if NET == "" && INIT == true {
        kerror!("Net never set but was initialized, you should set a net driver");
        NET = "Unknown";
    }

    if NET == "ESP-WROOM-32" {
        esp32net::esp_wroom_32_init();
    } else {
        kerror!("{} is an unknown driver", NET);
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_net(name: &'static str) {
    NET = name;
}

#[no_mangle]
pub unsafe extern "C" fn set_net_init() {
    INIT = true;
}

#[no_mangle]
pub unsafe extern "C" fn net_name() -> &'static str {
    return NET;
}
