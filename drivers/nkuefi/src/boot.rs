use core::fmt::Write;
use super::{kernel, proto};
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};

extern "C" {
    fn boot_init();
}

#[no_mangle]
pub unsafe extern "C" fn efi_main(img: Handle, st: SystemTable<Boot>) -> ! {
    uefi_services::init(&st);

    st.stdout().reset(false).expect_success("Failed to reset stdout");

    writeln!(st.stdout(), "{}", "Starting kernel...");

    proto::proto_init(st);

    #[cfg(target_arch = "x86_64")]
    boot_init();

    kernel::start_novusk();
}
