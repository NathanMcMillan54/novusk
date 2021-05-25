use crate::include::asm::hlt;
use crate::kernel::{init, printk};
use uefi::proto::console::text::Input;
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn keyboard_init(stdin: *mut Input) {

}

pub unsafe fn bmain() -> ! {
    init::init();
    hlt()
}
