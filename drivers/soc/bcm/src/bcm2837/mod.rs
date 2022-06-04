use soc::info::SocInfo;

pub mod mailbox;
pub mod serial;

#[no_mangle]
pub static mut SOC_INFO: SocInfo = SocInfo {
    known: true,
    name: "Broadcom BCM2837",
    addresses: [
        ("Peripheral Address", 0x3F00_0000 as *mut u8),
        ("Video Core Offset", 0xB880 as *mut u8),
        ("Timer CS Offset", 0x3000 as *mut u8),
        ("IRQ Base Offset", 0xB210 as *mut u8),
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
