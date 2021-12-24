use crate::define_syscall;
pub use power::Power;

// ------------
// Reboot/sys_reboot
//
// System call for rebooting
fn reboot(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    let mut power = Power::new();
    power.reboot();

    return 1;
}

define_syscall!(sys_reboot, reboot);
