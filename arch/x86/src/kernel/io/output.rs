use core::fmt::Write;
use core::str::from_utf8_unchecked;
use uefi::proto::console::text::Output;

pub static mut STDOUT: Option<&mut Output> = None;

pub unsafe fn output_init() {
    STDOUT = Option::from(uefi_services::system_table().as_ref().stdout());
}

#[no_mangle]
pub unsafe extern "C" fn write(bytes: &[u8]) {
    printk!("{}", from_utf8_unchecked(bytes));
}
