// Cortex M startup
use super::main::bmain;

#[entry]
fn cortex_m_startup() -> ! {
    bmain();
}
