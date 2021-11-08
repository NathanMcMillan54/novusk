use crate::aarch64_printk;

extern "C" {
    static mut __bss_start: u64;
    static mut __bss_end: u64;
}

pub unsafe fn memory_map() -> (usize, usize) {
    return (__bss_end as usize, __bss_start as usize);
}

pub fn print_memory_map() {
    aarch64_printk!("_________________\n");
    aarch64_printk!("| Memory map    |\n");

    let (end, start) = unsafe { memory_map() };
    aarch64_printk!("_________________\n");
    aarch64_printk!("| Bss start: {}  |\n", start);
    aarch64_printk!("| Bss end: {}    |\n", end);
    aarch64_printk!("-----------------\n");
}
