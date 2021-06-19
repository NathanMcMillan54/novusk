use core::fmt::Write;
use super::kernel;
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub unsafe extern "C" fn efi_main(img: Handle, st: SystemTable<Boot>) -> ! {
    uefi_services::init(&st);

    st.stdout().reset(false).expect_success("Failed to reset stdout");

    writeln!(st.stdout(), "{}", "Starting kernel...");

    kernel::start_novusk();
}
