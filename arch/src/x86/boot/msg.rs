use crate::x86::kernel::vga_buffer::*;

pub fn boot_msg(msg: &str, pos: i32, color: Color) {
    let mut writer = Writer {
        column_position: pos as usize,
        color_code: ColorCode::new(color, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };
    writer.write_string(msg);
}
