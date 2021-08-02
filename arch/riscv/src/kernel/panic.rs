use core::panic::PanicInfo;
use crate::include::asm::wfi;
use crate::riscv_printk;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    riscv_printk!("Kernel panicked");
    wfi();
}
