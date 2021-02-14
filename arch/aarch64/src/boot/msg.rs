use crate::drivers::uart;
use core::ptr;
use crate::kernel::out_str;

fn write_spaces(spaces: i32) {
    let mut bytewriter = out_str::Writer;
    bytewriter.write_byte(b" ");
}

pub fn boot_msg(msg: &str, pos: i32) {
    let mut spaces = 0;
    let mut writer = out_str::Writer;
    if pos == 0 { /* Nothing */ } else { write_spaces(pos); }
    writer.write_string(msg);
}
