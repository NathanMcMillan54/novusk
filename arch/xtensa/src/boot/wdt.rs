use device::Device;
use crate::board::get_board;

#[derive(PartialEq)]
pub enum WdtModes {
    Disable,
    Enable,
}

pub fn set_watchdog_timer(mode: WdtModes) {
    if mode == WdtModes::Disable {
        get_board().disable_wdt();
    } else { get_board().disable_wdt(); }
}

