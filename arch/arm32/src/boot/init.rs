use super::cpu::cpu_init;
use cortex_m_rt_macros::entry;

#[entry]
fn init() -> ! {
    unsafe {
        cpu_init();
    }

    panic!("Kernel ended");
}
