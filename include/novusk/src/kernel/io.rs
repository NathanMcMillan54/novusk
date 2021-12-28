use core::str::from_utf8_unchecked;
use crate::define_syscall;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/kernel/uart.rs"]
pub mod a64_io;

/*cfg_if! {
    if #[cfg(target_arch = "arm")] {
        fn a32_write(sys_arg: u8) -> u8 {
            unsafe { printk::printk!("{}", from_utf8_unchecked(sys_arg.to_be_bytes().as_ref())); }
            return 0;
        }

        fn a32_read(sys_arg: u8) -> u8 {
            return 1;
        }

        define_syscall!(WRITE, sys_write, 4, a32_write);
        define_syscall!(READ, sys_read, 3, a32_read);
    }
}
*/