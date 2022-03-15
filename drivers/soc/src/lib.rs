#![no_std]

use core::default::Default;

pub type SocAddr = (&'static str, *mut u8);

#[derive(PartialEq)]
pub struct SocInfo {
    pub known: bool,
    pub name: &'static str,
    pub addresses: [SocAddr; 11],
}

impl Default for SocInfo {
    fn default() -> Self {
        return SocInfo {
            known: false,
            name: "Unknown - not set",
            addresses: [
                ("Peripheral Base", 0x0 as *mut u8),
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
    }
}

pub unsafe fn validate_soc_dif(dif_periph_addr: *mut u8) -> bool {
    extern "C" {
        static mut SOC_INFO: SocInfo;
    }

    let soc_addresses = SOC_INFO.addresses;
    let mut ret = false;

    if SOC_INFO == SocInfo::default() {
        ret = true;
        return ret;
    }

    for (name, addr) in soc_addresses {
        if name == "Peripheral Base" {
            if addr == dif_periph_addr {
                ret = true;
            }
        }
    }

    return ret;
}

