pub use crate::{arm32_printk, dprint};
use alloc::vec;

#[no_mangle]
pub unsafe extern "C" fn start_kernel() {
    kinfo!("Kernel initialized");
}
