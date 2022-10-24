#![no_std]

#[macro_use] extern crate novuskinc;

use novuskinc::module::*;

use x86_64::instructions::port::Port;

pub mod input;
pub mod types;

pub fn _init_kb_mouse(km: &mut KernelModule) {
    // printk!("Kb Mouse init");

    let byte: u8 = unsafe { Port::new(0x60).read() };
    let byte_option = Some(byte);

    if byte_option.is_none() {
        panic!("Keyboard port not found, cannot use Kb Mouse");
    }
}

module_init!(ModuleType::InKernel, kb_mouse);

pub fn _end_kb_mouse(km: &mut KernelModule) {
    // printk!("Kb Mouse end");
}

module_end!(ModuleType::InKernel, kb_mouse);
