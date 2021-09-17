use crate::Power;

#[cfg(target_arch = "x86_64")]
use super::x64_power::reboot;

#[cfg(target_arch = "aarch64")]
use super::a64_power::reboot;

#[cfg(target_arch = "arm")]
use super::a32::reboot;

impl Power {
    pub fn reboot(&mut self) -> ! {
        unsafe { reboot() }
    }
}
