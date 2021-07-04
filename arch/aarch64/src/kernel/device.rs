pub static mut MMIO_BASE: *mut u8 = 0 as *mut u8;

#[derive(Copy, Clone, PartialEq)]
pub enum Board {
    None,
    Virt,
    RPi3,
    RPi4,
}
