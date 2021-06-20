use super::kernel::*;
use super::x86_init::x86_init;
use crate::boot::boot::{boot_init, BOOT};

#[no_mangle]
pub unsafe extern "C" fn x86_main() -> ! {
    x86_printk!("");

    kinfo!("Boot initialized");
    x86_printk!("   Boot method: {}", BOOT);

    x86_init();
    loop {  }
}
