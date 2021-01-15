use super::startKernel;

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    unsafe { startKernel() }
}
