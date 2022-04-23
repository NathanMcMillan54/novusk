#![no_std]

#[macro_use] extern crate novuskinc;

use core::panic::PanicInfo;
use soc::info::*;

#[no_mangle]
pub static mut SOC_INFO: SocInfo = SocInfo {
    known: true,
    name: "Broadcom BCM2837",
    addresses: [
        ("Peripheral Base", 0x3F00_0000 as *mut u8),
        ("Video Core Address", 0x3F00_B880 as *mut u8),
        ("Timer CS Address", 0x3F00_3000 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
    ]
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}

