use crate::drivers::vga_text::{BUFFER_HEIGHT};
use crate::kernel::vga_buffer::{*};

pub fn setup_cmdline() {
    let mut line = 0;
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    while line != BUFFER_HEIGHT {
        writer.write_byte(b'\n');
        line = line + 1;
    }
}
