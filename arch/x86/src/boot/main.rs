use super::{cmdline, dev::device_init};
use crate::include::asm::hlt;
use crate::kernel::{init, printk, userspace::early};
use uefi::proto::console::text::Input;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn keyboard_init(stdin: *mut Input) {

}

pub unsafe fn bmain() -> ! {
    device_init();
    early::early_user_init();
    printk::printk_init();
    init::init();
    hlt()
}
