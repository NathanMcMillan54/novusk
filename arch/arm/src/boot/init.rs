use crate::boot::die;
use crate::kernel::device::device_init;
use crate::kernel::kernel::{start_kernel, arm32_printk};
use crate::mm::init::arm32_memory_init;
use cortex_m_rt::entry;

unsafe fn init() {
    arm32_memory_init();
    kinfo!("Memory initialized");
    device_init();
    kinfo!("Device initialized");
    start_kernel();
    arm32_printk!("");
    kinfo!("arm(32) kernel initialized");
    arm32_printk!("    Ending kernel...");
}

#[entry]
fn main() -> ! {
    arm32_printk!("Starting kernel...");
    arm32_printk!("");

    unsafe {
        init();

        die();
    }
}
