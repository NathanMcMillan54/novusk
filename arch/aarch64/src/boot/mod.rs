pub mod setup;

use core::ptr::write_volatile;
use self::setup::Aarch64Boot;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() {
    for b in b"Starting Aarch64 kernel\n" {
        write_volatile(0x3F20_1000 as *mut u8, *b);
    }

    let aarch64_boot = Aarch64Boot::new();

    aarch64_boot.setup();
}
