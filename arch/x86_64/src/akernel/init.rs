use super::syscalls::write_fmt;

pub unsafe fn init() {
    printk!("x86_64 kernel init\n");
}
