use crate::modules::KernelModules;

pub unsafe fn run(module: KernelModules) {
    #[cfg(target_arch = "x86_64")]
    match module {
        KernelModules::Ex1 =>
            ex1::ex1_init(),
        KernelModules::None =>
            return,
    }
    printk!("    {:?} initialized", module);
}

pub unsafe fn arch_modules_init(modules: &[KernelModules; 10]) {
    for i in 0..10 {
        run(modules[i]);
    }
}
