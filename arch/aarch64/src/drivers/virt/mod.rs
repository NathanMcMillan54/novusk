// Drivers for the virt QEMU board
pub mod info;
pub mod power;
mod setup;
pub mod uart;


pub use setup::virt_setup;
