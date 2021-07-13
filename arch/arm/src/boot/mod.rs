use crate::include::asm::wfe;
use crate::kernel::board::led::blink;

#[cfg(target_arch = "arm")]
#[cfg(feature = "nrf")]
pub mod init;

#[no_mangle]
pub unsafe extern "C" fn die() -> ! {
    blink();
    blink();

    panic!("Kernel died nothing to run");

    wfe();
}
