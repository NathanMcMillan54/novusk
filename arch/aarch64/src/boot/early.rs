use crate::kernel::uart::uart_init;

unsafe fn memory_setup() {
    extern "C" {
        static mut __bss_start: u64;
        static mut __bss_end: u64;
    }

    r0::zero_bss(&mut __bss_start, &mut __bss_end);
}

pub unsafe fn early_aarch64_init() {
    memory_setup();
    uart_init();
}
