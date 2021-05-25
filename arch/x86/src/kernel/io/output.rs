use core::fmt::Write;
use uefi::proto::console::text::Output;

pub static mut STDOUT: Option<&mut Output> = None;

pub unsafe fn output_init() {
    STDOUT = Option::from(uefi_services::system_table().as_ref().stdout());
}
