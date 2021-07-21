pub use crate::{arm32_printk, dprint};
use alloc::vec;
use core::panic::PanicInfo;
use super::board::led::blink;
use super::mb::{clear_mb, MAILBOX};
use crate::kernel::device::DEVICE;

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    clear_mb();
    kinfo!("MailBox cleared");
    arm32_printk!("    MB: {:?}", MAILBOX);
    kinfo!("Kernel initialized\n");
    //blink();

    arm32_printk!("Starting userspace on {:?}... ", DEVICE);

    extern "C" { fn kernel_main(); }
    kernel_main();
}

#[cfg(target_arch = "arm")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    arm32_printk!("\nKernel panicked:");
    arm32_printk!("   Message: {:?}", _info.message().unwrap());
    arm32_printk!("   Location: {:?}", _info.location().unwrap());
    loop {  }
}
