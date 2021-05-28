use crate::board::rpi3_blink;
use crate::boot::reset;

#[no_mangle]
pub unsafe extern "C" fn restart() -> ! {
    for _ in 1..3 {
        rpi3_blink();
    }
    reset()
}
