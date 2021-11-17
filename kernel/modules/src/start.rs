use crate::modules::KernelModules;

pub(crate) unsafe fn run(module: KernelModules) {
    match module {
        KernelModules::None => return,
        KernelModules::Ex1 => {
            start_module!(ex1_init, ex1_end);
        },
        KernelModules::FsCheck => {
            start_module!(fscheck_init, fscheck_end);
        },
        _ => return,
    }

    printk!("    {:?} initialized\n", module);
}

pub fn arch_modules_init(modules: &[KernelModules; 10]) {
    for i in 0..10 {
        unsafe { run(modules[i]); }
    }
}

pub fn kernel_modules_init(modules: &[KernelModules; 10]) {
    for i in 0..10 {
        unsafe { run(modules[i]); }
    }
}
