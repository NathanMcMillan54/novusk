use printk::printk;
use crate::define_syscall;

// -----------------
// Read/sys_write for x86_64
//
// Read system call
#[cfg(target_arch = "x86_64")]
pub unsafe fn read(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    use pc_keyboard::PcKeyboard;

    let mut keyboard = PcKeyboard::new();
    let mut input = keyboard.read_bytes();

    return *input.as_ptr();
}

#[cfg(target_arch = "x86_64")]
define_syscall!(sys_read, read);
