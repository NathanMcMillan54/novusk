use super::setup;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    unsafe {
        setup::setup();
    }
}
