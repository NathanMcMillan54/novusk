use crate::kernel::device::Board;
use crate::kernel::mm::bss::zero_bss_se;
use crate::kernel::mm::mmio::Mmio;

pub(crate) unsafe fn rpi_memory_init() {
    // If clearing bss in the boot assembly doesn't change anything clear it here
    // zero_bss_se();

    // For now only RPi3 mmio will be setup
    let mut mmio = Mmio::mmio(&Mmio);
    mmio.mmio_init(Board::RPi3);
}
