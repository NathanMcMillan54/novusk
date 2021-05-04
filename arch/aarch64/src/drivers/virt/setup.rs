use crate::drivers::virt::power::shutdown;
global_asm!(include_str!("power/shutdown.S"));

pub unsafe fn virt_setup() -> ! {
    shutdown();
}
