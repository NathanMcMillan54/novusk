use super::syscalls::write_fmt;
// use drivers::x86_64::vga::WRITER;

pub unsafe fn init() {
    write_fmt(format_args!("{}{}", "x86_64 kernel ", "init\n"));
}
