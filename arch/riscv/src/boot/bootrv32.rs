use super::{die, main};
use crate::riscv_printk;

#[entry]
fn main() -> ! {
    unsafe { main::start_riscv_kernel(); }
}
