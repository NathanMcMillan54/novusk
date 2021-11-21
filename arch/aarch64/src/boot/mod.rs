global_asm!(include_str!("boot64.S"));

use crate::aarch64_printk;
use crate::kernel::init::aarch64_init;
use crate::kernel::serial;
use crate::include::asm::wfe;
use crate::mm::memory_init;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    memory_init();

    serial::serial_init();
    aarch64_printk!("Starting kernel...\n\n");

    aarch64_init();
    panic!("Nothing to run");
}
