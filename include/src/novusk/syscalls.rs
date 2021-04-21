use core::fmt::Arguments;

extern "C" { pub fn write_fmt(args: Arguments) -> Arguments; }
extern "C" { pub fn kernel_time() -> f64; }
