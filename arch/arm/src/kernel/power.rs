#[cfg(any(feature = "stellaris_6965"))]
pub use crate::cortex_m3::power::reboot;

#[cfg(any(feature = "nrf52840"))]
pub use crate::cortex_m4::power::reboot;
