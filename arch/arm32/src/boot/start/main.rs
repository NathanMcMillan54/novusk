use crate::boot::cpu::early_cpu_init;
use crate::boot::init::init_arm_kernel;
use core::ptr::write_volatile;

#[no_mangle]
pub extern "C" fn bmain() -> ! {
    early_cpu_init();

    init_arm_kernel();

    loop {  }
}
