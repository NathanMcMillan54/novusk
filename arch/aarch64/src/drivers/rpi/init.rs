// Main RPi 3 boot file
use arm_lib::include::asm::wfe;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    wfe()
}
