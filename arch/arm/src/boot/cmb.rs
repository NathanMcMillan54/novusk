use super::mem::{clear_bss};
use cortex_m::Peripherals;

extern "C" {
    static mut __sbss: *mut u32;
    static mut __ebss: *mut u32;
}

#[entry]
fn _cm_start() -> ! {
    unsafe {
        clear_bss(__sbss, __ebss);
    }

    crate::kernel::cm_kernel::start_cortex_m_kernel();

    //panic!("Kernel ended");
    loop {  }
}
