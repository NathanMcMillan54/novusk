use crate::boot::die;
use crate::kernel::device::device_init;
use crate::kernel::kernel::{start_kernel, arm32_printk};
use crate::mm::init::arm32_memory_init;
use cortex_m_rt::entry;

unsafe fn init() {
    arm32_memory_init();
    device_init();
    start_kernel();
}

#[entry]
fn main() -> ! {
    arm32_printk!("Starting kernel...");

    unsafe {
        init();

        die();
    }
}
