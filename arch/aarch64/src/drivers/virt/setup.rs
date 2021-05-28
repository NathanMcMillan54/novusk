global_asm!(include_str!("power/shutdown.S"));
global_asm!(include_str!("init.S"));

use crate::drivers::device::{device_init, Device, DEVICE_INFO};
use crate::drivers::uart::uart0::Uart0;
use crate::kernel::time::sleep::sleepm;
use crate::drivers::virt::io::early_write_byte;
use super::power::shutdown;
use crate::drivers::virt::info::UART0;

#[no_mangle]
pub unsafe extern "C" fn virt_init() {
    // Starting...\n
    early_write_byte(83);
    early_write_byte(116);
    early_write_byte(97);
    early_write_byte(114);
    early_write_byte(116);
    early_write_byte(105);
    early_write_byte(110);
    early_write_byte(103);
    early_write_byte(46);
    early_write_byte(46);
    early_write_byte(46);
    early_write_byte(10);

    let mut uart = Uart0;
    uart.uart0_init(UART0);
}
