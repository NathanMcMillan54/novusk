use crate::modules::KernelModules;

pub(crate) unsafe fn run(module: KernelModules) {
    match module {
        KernelModules::None => return,
        KernelModules::Ex1 => ex1::ex1_init(),
        KernelModules::FsCheck => fscheck::fscheck_init(),
        _ => return,
    }

    printk!("    {:?} initialized", module);
}

pub fn arch_modules_init(modules: &[KernelModules; 10]) {
    for i in 0..10 {
        unsafe { run(modules[i]); }
    }
}

pub unsafe fn kernel_modules_init(modules: &[KernelModules; 10]) {
    for i in 0..10 {
        unsafe { run(modules[i]) }
    }
}
