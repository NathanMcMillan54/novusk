use crate::kernel::device_init;
use crate::kernel::io::ARM32IO;
use crate::mm::allocator::allocator_init;

pub unsafe fn cortex_m_init() {
    allocator_init();
    let (init, name) = device_init();

    if init.is_err() {
        panic!("Failed to initialize device");
    }

    ARM32IO.lock().init("hio", name);
}
