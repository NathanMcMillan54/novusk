use core::fmt::Arguments;
use crate::printk_init;

pub static mut KERN_INFO: bool = true;

pub unsafe fn _kinfo(args: Arguments) {
    if !printk_init() {
        return;
    } else {
        info!("[ {} ] {}", KERN_INFO, args);
    }
}
