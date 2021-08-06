pub static mut MMIO_ADDRESS: u32 = 0;

pub unsafe fn mmio_init() {
    #[cfg(feature = "rpi3")]
    MMIO_ADDRESS = 0x3F000000;
}
