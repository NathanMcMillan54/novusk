#![no_std]

#[macro_use] extern crate novuskinc;

use x86_64::instructions::port::Port;

pub mod input;
pub mod types;

pub fn kb_mouse_start() {
    printk!("Kb Mouse init");

    let byte: u8 = unsafe { Port::new(0x60).read() };
    let byte_option = Some(byte);

    if byte_option.is_none() {
        panic!("Keyboard port not found, cannot use Kb Mouse");
    }
}

module_init!(kb_mouse_init, kb_mouse_start);

pub fn kb_mouse_finish() {
    printk!("Kb Mouse end");
}

module_end!(kb_mouse_end, kb_mouse_finish);
