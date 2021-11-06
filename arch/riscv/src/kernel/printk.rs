use core::fmt::Arguments;
use device::Device;
use crate::board::get_board;

#[no_mangle]
pub extern "C" fn arch_printk(fmt: Arguments) {
    let mut board = get_board();
    let bytes = fmt.as_str().unwrap().as_bytes();

    board.write_bytes(bytes);
}

#[no_mangle]
pub extern "C" fn _kernel_main_print(fmt: Arguments) {
    arch_printk(fmt);
}
