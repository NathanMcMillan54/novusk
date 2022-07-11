use soc::info::SocInfo;

pub mod mailbox;
pub mod serial;

#[no_mangle]
pub static mut SOC_INFO: SocInfo = SocInfo {
    known: true,
    name: "Broadcom BCM2837",
    addresses: [
        ("Peripheral Address", 0x3F00_0000),
        ("Video Core Offset", 0xB880),
        ("Timer CS Offset", 0x3000),
        ("IRQ Offset", 0xB210),
        ("GPIO Offset", 0x20_0000),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
        ("None", 0x0),
    ]
};
