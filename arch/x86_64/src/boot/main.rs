use kinfo::status::KStatus;
use super::cpu::validate_cpu;
use crate::early_printk;
use crate::kernel::setup::setup_x86_64;

fn test_alloc() {
    let mut test_vec = vec![0];

    for i in 0..1024 {
        test_vec.push(i);
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> ! {
    early_printk!("Starting kernel...\n\n");

    if !validate_cpu() {
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

    setup_x86_64();
    early_printk!("x86_64 kernel setup finished\n");

    panic!("x86_64 kernel ended");
}
