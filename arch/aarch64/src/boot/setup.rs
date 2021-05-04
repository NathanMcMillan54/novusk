use crate::drivers::{virt};

#[no_mangle]
pub unsafe extern "C" fn setup() -> ! {
    #[cfg(feature = "board_virt")]
    virt::virt_setup();

    loop {  }
    // loop { asm!("wfe"); }
}
