use kinfo::status::set_status;
use modules::modules::KernelModules;
use modules::start::arch_modules_init;
use crate::riscv_printk;

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
    crate::mm::mm_init();

    kinfo!("Starting RISCV kernel modules...");
    riscv_modules_init();

    libbmu::bmu_init();
}
