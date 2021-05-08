use core::fmt::Write;
use uefi::table::{SystemTable, Boot};
use uefi::proto::console::text::Output;
use uefi_kd::{UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION};

#[no_mangle]
pub unsafe extern "C" fn cmdline_init(st: SystemTable<Boot>) {
    st.stdout().reset(false).expect("Couldn't reset stdout");
    // writeln! only works in UEFI functions
    writeln!(st.stdout(), "Starting kernel...");
    writeln!(st.stdout(), "UEFI version: {}.{}", UEFI_MAJOR_VERSION, UEFI_MINOR_VERSION);
}
