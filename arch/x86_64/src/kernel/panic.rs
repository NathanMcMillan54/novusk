use core::arch::asm;
use core::panic::PanicInfo;
use crate::early_printk;
use crate::include::dif::DIF;
use crate::kernel::power::shutdown;

#[panic_handler]
unsafe fn _panic(_info: &PanicInfo) -> ! {
    early_printk!("\n\nx86_64 kernel panic\n");
    early_printk!("    Message: {}\n", _info.message().unwrap());
    early_printk!("    Location: {}\n", _info.location().unwrap());
    early_printk!("    Timer value: {}\n", crate::kernel::time::TIMER_VALUE);

    if DIF.get("ShutdownOnPanic").1.parse::<bool>().unwrap() {
        shutdown();
    }

    loop { asm!("hlt"); }
}
