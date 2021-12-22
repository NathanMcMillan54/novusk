use super::{Power, arch_power};

impl Power {
    pub fn reboot(&self) -> ! {
        unsafe { arch_power::reboot(); }
    }
}
