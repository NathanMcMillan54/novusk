use core::fmt::{Arguments, Write};
use uefi_services::system_table;

extern "C" {
    fn x86_main() -> !;
}

pub unsafe fn start_novusk() -> ! {
    #[cfg(target_arch = "x86_64")]
    x86_main();
}

#[link_name = "efi_print"]
#[no_mangle]
pub unsafe extern "C" fn _efi_println(fmt: Arguments) {
    let stdout = system_table().as_ref().stdout();
    writeln!(stdout, "{}", fmt);
}
