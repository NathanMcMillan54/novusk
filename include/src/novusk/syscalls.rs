use core::fmt::Arguments;

extern "C" { pub fn write_fmt(args: Arguments) -> Arguments; }
