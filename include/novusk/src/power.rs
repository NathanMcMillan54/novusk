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

define_syscall!(sys_reboot, reboot);
