use crate::board::get_board;
use crate::include::syscalls::syscalls_init;

pub fn setup_riscv_kernel() {
    let mut board = get_board();

    unsafe { syscalls_init(); }
    kinfo!("System calls initialized\n");
}
