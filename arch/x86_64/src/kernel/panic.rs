use core::panic::PanicInfo;
use crate::early_printk;

#[panic_handler]
unsafe fn _panic(_info: &PanicInfo) -> ! {
    early_printk!("x86_64 kernel panic\n");
    early_printk!("Message: {}\n", _info.message().unwrap());
    early_printk!("Location: {}\n", _info.location().unwrap());

    loop { asm!("hlt"); }
}
