use super::mem::{clear_bss};
use cortex_m::Peripherals;

extern "C" {
    static mut __sbss: *mut u32;
    static mut __ebss: *mut u32;
}

#[entry]
fn _cm_start() -> ! {
    crate::kernel::cm_kernel::start_cortex_m_kernel();

    //panic!("Kernel ended");
    loop {  }
}
