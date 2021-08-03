use crate::modules::KernelModules;

const MODULE_PATH: &str = "/libs/kmod/";

pub unsafe fn run(module: KernelModules) {
    match module {
        KernelModules::None =>
            return,

        KernelModules::Ex1 =>
            ex1::ex1_init(),
    }

    printk!("    {:?} initialized", module);
}

pub unsafe fn arch_modules_init(modules: &[KernelModules; 10]) {
    for i in 0..10 {
        run(modules[i]);
    }
}
