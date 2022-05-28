use soc::info::SocInfo;

pub mod serial;

#[no_mangle]
pub static mut SOC_INFO: SocInfo = SocInfo {
    known: true,
    name: "Broadcom BCM2837",
    addresses: [
        ("Peripheral Base", 0x3F00_0000 as *mut u8),
        ("Video Core Address", 0x3F00_B880 as *mut u8),
        ("Timer CS Address", 0x3F00_3000 as *mut u8),
        ("IRQ Base Address", 0x3F00_0B210 as *mut u8),
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
