use super::linker_mem::clear_bss_se;

pub unsafe fn arm32_memory_init() {
    extern "C" {
        static mut __ebss: u64;
        static mut __sbss: u64;
    }
    // Setup memory
    clear_bss_se(__sbss, __ebss);
}
