use alloc::string::String;
use konfig::Konfig;
use modules::{arch_modules_init, kernel_modules_init, KernelModules};

fn string_to_kernelmodules(string: String) -> KernelModules {
    return match string.as_str() {
        "" => KernelModules::None,
        "None" => KernelModules::None,
        "Ex1" => KernelModules::Ex1,
        "FsCheck" => KernelModules::FsCheck,
        _ => KernelModules::None,
    }
}

pub unsafe fn modules_init(mut configs: Konfig) {
    let module1 = string_to_kernelmodules(configs.get("KERNEL", "MODULE1"));
    let module2 = string_to_kernelmodules(configs.get("KERNEL", "MODULE2"));
    let module3 = string_to_kernelmodules(configs.get("KERNEL", "MODULE3"));
    let module4 = string_to_kernelmodules(configs.get("KERNEL", "MODULE4"));
    let module5 = string_to_kernelmodules(configs.get("KERNEL", "MODULE5"));
    let module6 = string_to_kernelmodules(configs.get("KERNEL", "MODULE6"));
    let module7 = string_to_kernelmodules(configs.get("KERNEL", "MODULE7"));
    let module8 = string_to_kernelmodules(configs.get("KERNEL", "MODULE8"));
    let module9 = string_to_kernelmodules(configs.get("KERNEL", "MODULE9"));
    let module10 = string_to_kernelmodules(configs.get("KERNEL", "MODULE10"));

    kernel_modules_init(&[module1, module2, module3, module4, module5, module6, module7, module8, module9, module10]);
}
