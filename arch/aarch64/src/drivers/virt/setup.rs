global_asm!(include_str!("power/shutdown.S"));
global_asm!(include_str!("init.S"));
use super::info::*;
use super::power::shutdown;
use crate::drivers::device::{device_init, Device, DEVICE_INFO};
use crate::drivers::uart::uart0::dprint::_dprint;
use crate::drivers::uart::uart0::Uart0;
use crate::kernel::time::sleep::sleepm;
use crate::drivers::virt::io::early_write_byte;
use core::fmt::Write;

#[no_mangle]
pub unsafe extern "C" fn virt_init() {
    let mut writer = Uart0;
    writer.write_string("Starting Aarch64 Qemu Virt kernel...\n");
    //_dprint(format_args!("{}", "formatted text!"));
}

