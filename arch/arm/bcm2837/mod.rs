use crate::early_printk;
use crate::include::dif::dif::DIF;

pub const BCM2837_PERIPHERAL_ADDRESS: *mut u8 = 0x3F00_0000 as *mut u8;

#[no_mangle]
pub unsafe extern "C" fn soc_init() -> i32 {
    if DIF.peripheral_addr.is_some() {
        if DIF.peripheral_addr.unwrap() as *mut u8 != BCM2837_PERIPHERAL_ADDRESS {
            return 1;
        }
    } else {
        early_printk!("Can't tell if device is using proper SOC driver\n");
        return 1;
    }

    return 0;
}
