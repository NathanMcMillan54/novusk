use core::fmt::Arguments;
use device::Device;
use crate::board::get_board;

#[no_mangle]
pub extern "C" fn arch_printk(fmt: Arguments) {
    let board = get_board();
}


