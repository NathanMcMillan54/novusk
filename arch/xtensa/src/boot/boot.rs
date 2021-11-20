use super::wdt::{set_watchdog_timer, WdtModes};
use crate::{kernel::setup::setup_xtensa, xprintk};

#[entry]
fn main() -> ! {
    set_watchdog_timer(WdtModes::Disable);
    setup_xtensa();

    loop { }
}
