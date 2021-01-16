use arch::ARCH;
use core::fmt::{Arguments, Write};

fn x86kprint(args: Arguments) {
    use arch::x86::kernel::vga_buffer::*;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_fmt(args).unwrap();
}

pub fn _kprint(arg: Arguments) {
    if ARCH == "x86" {
        x86kprint(arg);
    }
}

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => {$crate::kprint::_kprint(format_args!($($arg)*))};
}
