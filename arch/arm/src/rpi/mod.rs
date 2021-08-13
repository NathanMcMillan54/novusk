use crate::mm::linker_mem;

pub const MMIO_BASE: u32 = 0x3F00_0000;

#[no_mangle]
pub unsafe extern "C" fn aarch64_rpi_setup() {
    extern "C" {
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    r0::zero_bss(&mut __bss_start, &mut __bss_end);

    // Set addresses and drivers for RPi 3/4
    assert_eq!(MMIO_BASE, 0x3F00_0000);
}
