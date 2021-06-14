use core::fmt::Arguments;

extern "C" {
    pub(crate) fn arch_printk(args: core::fmt::Arguments);
}

pub unsafe fn _printk(fmt: Arguments) {
    arch_printk(format_args!("{}{}", fmt, "\n"));
}
