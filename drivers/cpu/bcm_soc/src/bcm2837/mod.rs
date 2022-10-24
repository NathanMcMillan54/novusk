use cpu::soc_info::SocInfo;

pub mod gpio;
pub mod mb;
pub mod uart;

#[no_mangle]
pub static SOC_INFO: SocInfo = SocInfo {
    name: "BCM2837",
    addresses: &[
        ("Peripheral", 0x3F00_0000),
        ("GPU", 0x0000_B880),
        ("Mini Uart", 0x0021_5000),
        ("GPIO", 0x0020_0000),
        ("", 0),
        ("", 0),
        ("", 0),
        ("", 0),
        ("", 0),
        ("", 0),
    ],
};
