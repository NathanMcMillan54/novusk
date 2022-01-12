use console::konsole::KernelConsole;
use konfig::Konfig;
use crate::drivers::*;
use crate::kmain::start_kernel_main;

pub fn display_init() {
    fb::fb_init();

    let console = KernelConsole::new((300, 100), 0x000000);
    console.display_kernel_console();
}

#[no_mangle]
pub unsafe extern "C" fn rpi2_kernel_init() {
    display_init();
    kinfo!("Frame buffer and console initialized\n");

    start_kernel_main();
    panic!("RPi 2 kernel finished");
}
