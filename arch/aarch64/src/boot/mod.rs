use crate::kernel::init::aarch64_kernel_init;
use core::arch::global_asm;
use kinfo::{InfoDisplay, status::KStatus};

mod early;
use early::early_aarch64_init;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    early_aarch64_init();
    early_printk!("Setup early memory\n");
    early_printk!("Initialized UART I/O\n");

    early_printk!("Starting kernel...\n\n");
    aarch64_kernel_init();

    panic!("Nothing to run");
}

global_asm!(include_str!("boot64.S"));
