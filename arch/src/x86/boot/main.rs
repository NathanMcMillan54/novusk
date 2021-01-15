use super::startKernel;
use super::super::kernel::init::x86_init;

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    unsafe { x86_init(); }
    unsafe { startKernel() }
}
