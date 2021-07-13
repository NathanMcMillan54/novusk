pub use crate::{arm32_printk, dprint};
use alloc::vec;
use core::panic::PanicInfo;

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    kinfo!("Kernel initialized");
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        arm32_printk!("\nKernel panicked:");
        arm32_printk!("   Message: {:?}", _info.message().unwrap());
        arm32_printk!("   Location: {:?}", _info.location().unwrap());
    }
    loop {  }
}
