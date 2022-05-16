pub mod kernel;
pub mod setup;

use self::kernel::start_aarch64_kernel;
use self::setup::Aarch64Boot;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() {
    let aarch64_boot = Aarch64Boot::new();

    aarch64_boot.setup();

    crate::early_printk!("Made it this far\n");

    start_aarch64_kernel();

    panic!("Boot setup should never return");
}
