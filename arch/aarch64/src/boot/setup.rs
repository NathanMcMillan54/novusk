#[cfg(feature = "board_virt")]
use crate::drivers::{virt};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {

    loop {  }
    // loop { asm!("wfe"); }
}
