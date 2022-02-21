#![no_std]

use core::default::Default;

pub type SocAddr = (&'static str, *mut u8);

pub struct SocInfo {
    pub known: bool,
    pub name: &'static str,
    pub addresses: [SocAddr; 11],
}

pub unsafe fn validate_soc_dif(dif_periph_addr: *mut u8) -> bool {
    extern "C" {
        static mut SOC_INFO: SocInfo;
    }

    let soc_addresses = SOC_INFO.addresses;
    let mut ret = false;

    for (name, addr) in soc_addresses {
        if name == "Peripheral Base" {
            if addr == dif_periph_addr {
                ret = true;
            }
        }
    }

    return ret;
}
