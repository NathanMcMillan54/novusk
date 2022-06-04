use core::fmt::Write;
use core::ptr::write_volatile;
use super::SOC_INFO;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::serial::SimpleUart;

#[export_name = "KERNEL_SIMPLEUART"]
#[no_mangle]
pub static mut BCM2837_SIMPLEUART: SimpleUart = SimpleUart::empty();

unsafe fn bcm2837_simpleuart_init() -> u8 {
    let peripheral_addr = SOC_INFO.get("Peripheral Address").unwrap() as *mut u8;

    BCM2837_SIMPLEUART.set_addrs(peripheral_addr.offset(0x0020_1000 as isize), 0x0 as *mut u8);

    0
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, bcm2837_simpleuart_init);
