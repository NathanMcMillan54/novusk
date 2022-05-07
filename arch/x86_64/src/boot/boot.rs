use super::cpu::{gdt, validate};
use kinfo::status::KStatus;
use crate::early_printk;

pub unsafe fn die() -> ! {
    early_printk!("\nNothing left to run\n");
    panic!("Kernel died");
}

fn test_alloc() {
    let mut test_vec = vec![0];

    for i in 0..1024 {
        test_vec.push(i);
    }
}

#[no_mangle]
pub unsafe extern "C" fn boot_init() {
    if !validate::validate_cpu() {
        kinfo!(KStatus {
            status: "not ok",
            should_panic: false,
            panic_message: None,
            message1: "Failed to validate CPU",
            message2: Some("This could cause errors later"),
        });
    } else {
        early_printk!("Running on a valid CPU\n");
    }

    test_alloc();
    early_printk!("Memory allocator tested\n");
}
