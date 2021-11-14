use crate::arm32_printk;
use crate::mm::arm_mm_init;
use init::kmain;
use setup::after_kernel_setup;
use cortex_m::peripheral::Peripherals;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::exception;

pub unsafe fn setup_arm32_kernel() {
    arm_mm_init();
    kinfo!("Memory initialized\n");
    arm32_printk!("    Allocator initialized\n");

    kmain::kernel_init();
    kinfo!("Novusk initialized\n");

    arm32_printk!("Starting after kernel...\n");
    after_kernel_setup();

    let peripherals = Peripherals::steal();
    let mut syst = peripherals.SYST;

    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000);
    syst.enable_counter();
    syst.enable_interrupt();

}
