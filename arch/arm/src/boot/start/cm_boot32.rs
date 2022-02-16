use cortex_m_rt::entry;
use super::start_boot;

#[entry]
fn main() -> ! {
    cortex_m_semihosting::hprintln!("Test");
    unsafe { start_boot(); }
}
