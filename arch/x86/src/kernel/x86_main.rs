use super::kernel::*;
use super::x86_init::x86_init;
use crate::boot::boot::boot_init;
use nkuefi::kernel::_efi_println;

#[no_mangle]
pub unsafe extern "C" fn x86_main() -> ! {
    boot_init();
    x86_printk!("x86 Main");

    x86_init();
    loop {  }
}
