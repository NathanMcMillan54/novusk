use core::fmt::Write;
use crate::kernel::start_novusk;
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub unsafe fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&system_table);

    system_table.stdout().reset(false).expect_success("Failed to reset stdout");

    writeln!(system_table.stdout(), "{}", "Starting kernel...");

    start_novusk();
}
