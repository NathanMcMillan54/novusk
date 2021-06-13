use core::fmt::Write;
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub extern "C" fn efi_main(image: Handle, st: SystemTable<Boot>) -> ! {
    uefi_services::init(&st);

    st.stdout().reset(false).expect_success("Couldn't reset display");

    writeln!(st.stdout(), "Starting kernel...");

    loop {  }
}
