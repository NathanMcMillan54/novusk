use soc::info::validate_soc_dif;
use crate::include::dif::dif::DIF;

pub(crate) unsafe fn soc_init() -> i32 {
    if validate_soc_dif(DIF.peripheral_addr.unwrap() as *mut u8) {
        return 0;
    } else { return 1; }
}
