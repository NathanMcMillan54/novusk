use super::kernel::writes;
use core::str::from_utf8_unchecked;

pub use init::version::kernel_version;
pub use super::reboot::reboot;

pub fn write(write: u8) -> u8 {
    unsafe { writes(from_utf8_unchecked(vec![write].as_slice())); }
    return 0;
}

pub fn read(buf: u8) -> u8 {
    // TODO: "Read" from ps2/pc keyboard
    return buf;
}

define_syscall!(sys_write, write);
