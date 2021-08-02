use crate::mm::init::mm_init;

pub unsafe fn riscv_init() {
    kinfo!("Initializing and checking memory...");
    mm_init();
    kinfo!("RISCV memory initialized");
}
