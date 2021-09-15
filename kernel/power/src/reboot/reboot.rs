use crate::Power;

#[cfg(target_arch = "x86_64")]
use super::x64_power::reboot;

impl Power {
    pub fn reboot(&mut self) -> ! {
        unsafe { reboot(); }
    }
}
