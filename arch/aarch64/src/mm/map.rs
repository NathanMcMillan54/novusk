extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

pub unsafe fn memory_map() -> (usize, usize) {
    return (__bss_end as usize, __bss_start as usize);
}
