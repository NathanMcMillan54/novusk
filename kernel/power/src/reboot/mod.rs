pub mod reboot;

#[cfg(target_arch = "x86_64")]
#[path = "../../../../arch/x86_64/src/kernel/reboot.rs"]
pub mod x64_power;
