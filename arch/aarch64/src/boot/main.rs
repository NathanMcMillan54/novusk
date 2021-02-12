use crate::kernel::init::aarch64_end_kernel;

#[no_mangle]
pub extern "C" fn initStart() -> ! {
    aarch64_end_kernel()
}
