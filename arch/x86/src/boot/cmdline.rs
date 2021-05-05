use uefi::table::{SystemTable, Boot};
use uefi::proto::console::text::Output;
use core::fmt::Write;

#[no_mangle]
pub extern "C" fn cmdline_init(stdout: &mut Output) {
    stdout.clear().unwrap().unwrap();
    write!(stdout, "\t").unwrap();
}
