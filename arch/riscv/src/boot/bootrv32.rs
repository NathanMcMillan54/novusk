use super::{die, main};
use crate::riscv_printk;
use crate::kernel::board::Board;

#[entry]
fn main() -> ! {
    unsafe { main::start_riscv_kernel(); }
}
