use core::fmt::Arguments;
use libn::color::{vga};

extern "C" { pub fn write_fmt(args: Arguments) -> Arguments; }
extern "C" { pub fn kernel_time() -> f64; }
#[cfg(target_arch = "x86_64")]
extern "C" { pub fn draw(color: vga::Color) -> vga::Color; }
