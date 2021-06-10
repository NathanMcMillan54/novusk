use crate::include::asm::hlt;
use crate::kernel::{init, printk};
use uefi::proto::console::text::Input;
use uefi::table::{Boot, SystemTable};
use uefi_kd::text::{current_text_mode, set_text_mode};
use libefi::st;

pub unsafe fn cmdline_init() {
    set_text_mode(st().as_ref().stdout());
}

// Boot main
pub unsafe fn bmain() {
    cmdline_init();
    let mode = current_text_mode(st().as_ref().stdout()).unwrap();
    kinfo!("Kernel cmdline initialized");
    printk!("   Current text mode: {:?}", mode);
}
