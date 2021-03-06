use crate::boot::die;
use crate::kernel::device::device_init;
use crate::kernel::kernel::{start_kernel, arm32_printk};
use crate::mm::init::arm32_memory_init;
use cortex_m_rt::entry;

unsafe fn init() -> ! {
    arm32_memory_init();
    device_init();
    start_kernel();
    arm32_printk!("");
    kinfo!("arm(32) kernel initialized");
    arm32_printk!("    Ending kernel...");

    die();
}

#[entry]
fn main() -> ! {
    arm32_printk!("Starting kernel...");
    arm32_printk!("");

    unsafe { init(); }
}
