core::arch::global_asm!(include_str!("syscall.S"));

extern "C" {
    pub(crate) fn semihost_syscall(nr: usize, arg: usize) -> usize;
}

pub(crate) fn syscall<T>(num: usize, args: &T) -> usize {
    unsafe { semihost_syscall(num, args as *const T as usize) }
}