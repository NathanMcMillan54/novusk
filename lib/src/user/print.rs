use core::fmt::{Arguments, Write};
use arch::ARCH;

pub fn _print(args: Arguments) {
    if ARCH == "x86" {
        use arch::x86::kernel::vga_buffer::*;
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::White, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        };
        writer.write_fmt(format_args!("{}\n", args)).unwrap();
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({
        $crate::user::print::_print(format_args!($($arg)*));
    })
}
