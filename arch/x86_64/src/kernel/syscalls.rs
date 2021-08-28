use super::kernel::writes;
use core::str::from_utf8_unchecked;

pub fn write(write: u8) {
    unsafe { writes(from_utf8_unchecked(vec![write].as_slice())); }
}

pub fn read(buf: u8) -> u8 {
    // TODO: "Read" from ps2 keyboard
    return buf;
}

define_syscall!(sys_write, write);
