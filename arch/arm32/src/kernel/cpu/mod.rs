use cpu::CPUINFO;

pub mod interrupts;

pub fn set_cpu_info(arch: &'static str, name: &'static str, address: Option<u32>) {
    unsafe { CPUINFO.init(arch, name, address); }
}

fn get_cpu_info() -> (&'static str, &'static str, Option<u32>) {
    return ("ARM32", "Cortex-m", None);
}

pub fn cpu_kernel_init() {
    let (arch, name, address) = get_cpu_info();
    set_cpu_info(arch, name, address);
}
