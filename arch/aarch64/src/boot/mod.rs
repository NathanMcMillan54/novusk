global_asm!(include_str!("boot64.S"));

use crate::aarch64_printk;
use crate::kernel::init::aarch64_init;
use crate::kernel::uart::Uart;
use crate::include::asm::wfe;
use crate::mm::memory_init;
use rpi::aarch64_rpi_init;

#[no_mangle]
pub unsafe extern "C" fn aarch64_boot_setup() -> ! {
    memory_init();

    #[cfg(feature = "rpi3")]
    aarch64_rpi_init(3);

    let mut uart = Uart::new();
    uart.init();

    aarch64_printk!("Uart I/O initialized\n");
    aarch64_printk!("Starting kernel...\n\n");

    aarch64_init();
    panic!("Nothing to run");
}
