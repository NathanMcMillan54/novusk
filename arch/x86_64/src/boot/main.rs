use kinfo::status::KStatus;
use super::boot::boot_init;
use super::cpu::validate::validate_cpu;
use crate::early_printk;
use crate::kernel::{setup, x86_init};

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    boot_init();
    early_printk!("Starting kernel...\n\n");

    setup::setup_x86_64();
    early_printk!("x86_64 kernel setup finished\n");
    early_printk!("Initializing x86_64 kernel...\n");

    x86_init::x86_kernel_init();
    early_printk!("x86_64 kernel initialized\n");

    panic!("x86_64 kernel ended");
}
