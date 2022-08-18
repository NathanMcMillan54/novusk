use core::fmt::Arguments;
use crate::status::{set_status, KSTATUS};

pub fn _kinfo(msg: Arguments) {
    unsafe {
        printk!("INFO [ {} ] {}", KSTATUS, msg);
        set_status("ok");
    }
}

/// ``kinfo`` displays anything that implements ``InfoDisplay``
#[macro_export]
macro_rules! kinfo {
    ($info:expr) => {
        $info.display_info();
    };
}
