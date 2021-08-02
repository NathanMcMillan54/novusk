use kinfo::info::set_info;
use super::bss;
use crate::riscv_printk;

pub unsafe fn mm_init() {
    let mut bss_memory = bss::BssInfo::new();
    let (mut sbss, mut ebss) = bss_memory.bss_values();

    if sbss < ebss {
        set_info("not ok");
        riscv_printk!("    bss start is less than bss end");
        riscv_printk!("    sbss {} < ebss {}", sbss, ebss);
        r0::zero_bss(&mut sbss, &mut ebss);

        if sbss < ebss {
            panic!("BSS start and end can't be cleared");
        }
    } else {
        riscv_printk!("    bss start is grater than bss end");
    }
}
