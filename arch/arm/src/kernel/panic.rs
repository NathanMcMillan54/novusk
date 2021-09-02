use crate::include::asm::wfe;
use crate::kernel::early_printk::EARLYPRINTK;
use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    EARLYPRINTK.lock().write_fmt(format_args!("{:?}", _info));

    unsafe { wfe() }
}
