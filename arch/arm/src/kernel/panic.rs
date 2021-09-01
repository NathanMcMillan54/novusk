use crate::include::asm::wfe;
use core::panic::PanicInfo;
use crate::boot::hio::hio_write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hio_write(format_args!("{:?}", _info));
    unsafe { wfe() }
}
