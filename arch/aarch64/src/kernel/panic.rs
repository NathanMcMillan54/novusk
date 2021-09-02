use crate::aarch64_printk;
use crate::include::asm::wfe;
use core::panic::PanicInfo;

#[panic_handler]
pub unsafe fn panic(_info: &PanicInfo) -> ! {
    aarch64_printk!("\nAarch64 kernel panicked");
    aarch64_printk!("    Message: {:?}", _info.message().unwrap());
    aarch64_printk!("    Location: {:?}", _info.location().unwrap());
    wfe()
}
