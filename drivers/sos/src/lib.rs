#![no_std]

#[macro_use] extern crate printk;


pub fn sos_init(os_name: &str) {
    if os_name == "FOMOS" {
        fom_os::fom_os_init();
    } else {
        printk!("Couldn't find {} OS", os_name);
    }
}
