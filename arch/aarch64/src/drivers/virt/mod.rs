// Drivers for the virt QEMU board
pub mod info;
pub mod io;
pub mod power;
mod setup;
pub mod uart;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}

