use crate::aarch64_printk;
use crate::kernel::init::aarch64_kernel_init;
use core::arch::global_asm;

mod early;
use early::early_aarch64_init;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    early_aarch64_init();
    kinfo!("Early kernel initialized\n");
    aarch64_printk!("    Setup early memory\n");
    aarch64_printk!("    Initialized UART I/O\n");

    aarch64_printk!("Starting kernel...\n\n");
    aarch64_kernel_init();

    panic!("Nothing to run");
}

global_asm!(include_str!("boot64.S"));
