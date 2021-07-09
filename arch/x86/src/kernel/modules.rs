use super::kernel::x86_printk;
use modules::modules::KernelModules;
use modules::start::arch_modules_init;

pub unsafe fn x86_modules_init() {
    x86_printk!("Running x86 kernel modules...");
    arch_modules_init(&[KernelModules::Ex1, KernelModules::None, KernelModules::None, KernelModules::None, KernelModules::None, KernelModules::None, KernelModules::None, KernelModules::None, KernelModules::None, KernelModules::None]);
}
