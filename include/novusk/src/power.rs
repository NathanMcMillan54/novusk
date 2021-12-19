use crate::define_syscall;
// pub use power::Power;

// ------------
// Reboot/sys_reboot
//
// System call for rebooting
fn reboot(sys_arg: u8) -> u8 {
    // let mut power = Power::new();
    // power.reboot();
    return 1;
}

#[cfg(target_arch = "x86_64")]
define_syscall!(REBOOT, sys_reboot, 3, reboot);

#[cfg(target_arch = "aarch64")]
define_syscall!(REBOOT, sys_reboot, 142, reboot);

#[cfg(any(target_arch = "arm", target_arch = "riscv32"))]
define_syscall!(REBOOT, sys_reboot, 5, reboot);
