pub mod keyboard;
pub mod tests;

use crate::x86_printk;

pub unsafe fn ps2_init() {
    tests::ps2_test();
}
