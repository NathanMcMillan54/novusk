pub static mut MMIO_ADDRESS: u32 = 0;

pub unsafe fn mmio_init() {
    MMIO_ADDRESS = 1;
}
