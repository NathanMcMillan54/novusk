pub mod setup;

use core::ptr::write_volatile;
use self::setup::Aarch64Boot;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() {
    let aarch64_boot = Aarch64Boot::new();

    aarch64_boot.setup();
}
