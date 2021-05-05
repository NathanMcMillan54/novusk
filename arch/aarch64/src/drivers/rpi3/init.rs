// Main RPi 3 boot file
use arm_lib::include::asm::wfe;
use core::ptr::write_volatile;
use crate::drivers::rpi3::uart::UART0;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let mut text = "Starting Novusk on Aarch64 RPi 3\n";
    for chars in text.bytes() {
        write_volatile(UART0, chars);
    }
    wfe()
}
