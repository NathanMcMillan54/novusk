// Drivers for the Virt QEMU board
pub mod info;
pub mod io;
mod kernel;
pub mod power;
mod setup;
mod uart;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {  }
}

