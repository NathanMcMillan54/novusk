use crate::x86_printk;
use crate::boot::main::main;

#[no_mangle]
pub extern "C" fn grub_start_novusk() -> ! {
    x86_printk!("Booted with GRUB");
    loop {  }
}
