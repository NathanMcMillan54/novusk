use crate::define_syscall;

// -----------------
// Read/sys_write for x86_64
//
// Read system call
#[cfg(target_arch = "x86_64")]
pub fn read(sys_arg: u8) -> u8 {
    use pc_keyboard::PcKeyboard;

    let mut keyboard = PcKeyboard::new();
    let mut ret = 0;

    return ret;
}

#[cfg(target_arch = "x86_64")]
define_syscall!(sys_read, read);

