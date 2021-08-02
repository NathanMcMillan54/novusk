use core::panic::PanicInfo;
use crate::include::asm::wfi;
use crate::riscv_printk;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    riscv_printk!("\nKernel panicked:");

    riscv_printk!("    Info: {}", _info.message().unwrap());
    riscv_printk!("    Location: {}", _info.location().unwrap());

    wfi();
}
