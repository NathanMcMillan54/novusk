pub use sys::input;
use uefi::proto::console::text::Input;

pub static mut STDIN: Option<&mut Input> = None;

pub unsafe fn input_init() {
    STDIN = Option::from(uefi_services::system_table().as_ref().stdin());
}
