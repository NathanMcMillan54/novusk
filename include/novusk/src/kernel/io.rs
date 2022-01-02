use core::str::from_utf8_unchecked;
use crate::define_syscall;

#[cfg(target_arch = "aarch64")]
#[path = "../../../../arch/aarch64/src/kernel/uart.rs"]
pub mod a64_io;

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
#[path = "../../../../arch/riscv/src/kernel/serial.rs"]
pub mod risc_out;

fn a32_write(byte: u8, arg2: u8, arg3: u8) -> u8 {
    printk::printk!("{}", byte);
    return arg3;
}

#[cfg(target_arch = "arm")]
define_syscall!(sys_write, a32_write);
