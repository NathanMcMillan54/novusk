use core::arch::global_asm;

global_asm!("
.section .text.semihost_syscall
.global semihost_syscall

    .thumb_func

semihost_syscall:
    bkpt 0xAB
    bx lr
");

extern "C" {
    pub(crate) fn semihost_syscall(nr: usize, arg: usize) -> usize;
}

pub(crate) fn syscall<T>(num: usize, args: &T) -> usize {
    unsafe { semihost_syscall(num, args as *const T as usize) }
}
