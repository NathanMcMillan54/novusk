use crate::arm32_printk;
use crate::kernel::setup_arm32_kernel;
use super::init::early_init;

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    early_init();
    kinfo!("Early boot initialized\n");
    arm32_printk!("\nStarting kernel...\n");

    setup_arm32_kernel();
    kinfo!("Kernel is finished\n");

    panic!("Kernel ended");
}
