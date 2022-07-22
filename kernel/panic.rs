/// This file is for a global panic handler used throughout the kernel, any library/module that
/// needs a panic handler should be linked to this file.

use asminc::irq::ARCH_IRQS;
use core::panic::PanicInfo;
use printk::printk;

extern "C" {
    static KERNEL_NAME: &'static str;

    /// This function is used to indicate that the kernel panicked. If the device doesn't have a
    /// display this should flash an onboard LED.
    pub fn device_indicate_panic();

    /// Check the Dif to see if the kernel needs to shutdown after the panic.
    pub fn check_dif_panic();

    /// The ``arch_asm_loop`` function should come from [``asminc``](todo)
    pub fn arch_asm_loop();
}

/// The main panic handler
#[panic_handler]
pub unsafe fn _panic(info: &PanicInfo) -> ! {
    printk!("\n--- {} kernel panicked ---\n", KERNEL_NAME);
    printk!("| Getting ready to end...\n|");

    device_indicate_panic();
    ARCH_IRQS.enable_if_disabled();

    printk!("\n| Panic message: {}\n", info.message().unwrap_or(&format_args!("{}", "No message was given")));
    printk!("| Panic location: {}:{}\n", info.location().unwrap().file(), info.location().unwrap().line());
    printk!("| Finishing panic...\n");

    check_dif_panic();

    printk!("--- Ending...\n");

    arch_asm_loop();

    // In case arch_asm_loop returns for some reason
    loop {  }
}
