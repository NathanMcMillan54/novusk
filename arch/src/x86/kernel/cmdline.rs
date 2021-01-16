use crate::x86::include::time::sleep;
use crate::x86::kernel::vga_buffer::{*};
use crate::x86::drivers::vga_text::BUFFER_HEIGHT;

pub fn setup_cmdline() {
    let mut x = 0;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    while x != BUFFER_HEIGHT {
        writer.write_byte(b'\n');
        x = x + 1;
    }
}
