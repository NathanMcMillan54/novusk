use soc::*;

// SOC info for BCM28xx devices
#[no_mangle]
pub static mut SOC_INFO: SocInfo = SocInfo {
    known: true,
    name: "Broadcom, BCM28xx",
    addresses: [
        ("Peripheral Base", 0x3F00_0000 as *mut u8),
        ("UART0 Address", 0x3F20_1000 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
        ("None", 0x0 as *mut u8),
    ],
};
