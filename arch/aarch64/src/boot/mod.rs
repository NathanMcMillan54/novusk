global_asm!(include_str!("boot64.S"));

use crate::aarch64_printk;
use crate::kernel::init::aarch64_init;
use crate::kernel::uart::Uart;
use arm::rpi::aarch64_rpi_setup;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    aarch64_rpi_setup();

    let mut uart = Uart::new();
    uart.init();

    aarch64_printk!("Uart I/O initialized");
    aarch64_printk!("Starting kernel...\n");

    aarch64_init();

    loop { asm!("wfe"); }
}
