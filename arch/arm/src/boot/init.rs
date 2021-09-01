use crate::boot::hio::hio_write;
use crate::include::asm::wfe;
use crate::kernel::{device, setup};
use crate::mm::early_memory_init;

#[entry]
fn init() -> ! {
    unsafe {
        early_memory_init();
        device::device_init();
        setup::setup_kernel();
        wfe();
    }
}
