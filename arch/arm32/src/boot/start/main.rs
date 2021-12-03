use crate::arm32_printk;
use crate::boot::cpu::early_cpu_init;
use crate::boot::init::init_arm_kernel;
use crate::kernel::io::serial_io_init;

#[no_mangle]
pub extern "C" fn bmain() -> ! {
    unsafe { serial_io_init(); }
    arm32_printk!("Starting kernel...\n\n");

    early_cpu_init();

    init_arm_kernel();

    panic!("Nothing to run");
}
