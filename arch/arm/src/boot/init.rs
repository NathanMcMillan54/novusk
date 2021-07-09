use cortex_m_rt::entry;
use crate::boot::die;
use crate::kernel::device::device_init;
use crate::kernel::kernel::start_kernel;
use crate::mm::init::arm32_memory_init;

unsafe fn init() {
    arm32_memory_init();
    device_init();
    start_kernel();
}

#[entry]
fn main() -> ! {
    unsafe {
        init();

        die();
    }
}
