use super::{arch_power, Power};

impl Power {
    pub fn shutdown(&self) -> ! {
        unsafe { arch_power::shutdown(); }
    }
}
