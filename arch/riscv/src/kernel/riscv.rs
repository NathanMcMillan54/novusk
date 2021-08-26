use kinfo::status::set_status;
use modules::modules::KernelModules;
use modules::start::arch_modules_init;
use crate::riscv_printk;

#[cfg(not(feature = "no-mem"))]
unsafe fn mm_init() {
    let mut bss_memory = crate::mm::bss::BssInfo::new();
    let (mut sbss, mut ebss) = bss_memory.bss_values();

    if sbss < ebss {
        set_status("not ok");
        riscv_printk!("    bss start is less than bss end");
        riscv_printk!("    sbss {} < ebss {}", sbss, ebss);
        r0::zero_bss(&mut sbss, &mut ebss);

        if sbss < ebss {
            panic!("BSS start and end can't be cleared");
        }
    } else {
        riscv_printk!("    bss start is grater than bss end");
    }

    kinfo!("RISCV memory initialized");
}

unsafe fn riscv_modules_init() {
    arch_modules_init(&[
        KernelModules::Ex1,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
        KernelModules::None,
    ]);
}

pub unsafe fn riscv_init() {
    kinfo!("Initializing and checking memory...");

    #[cfg(not(feature = "no-mem"))]
    mm_init();

    kinfo!("Starting RISCV kernel modules...");
    riscv_modules_init();

    libbmu::bmu_init();
}
