use cpu::CPUINFO;

pub fn set_cpu_info(arch: &str, name: &str, address: Option<u32>) {
    unsafe { CPUINFO.init(arch, name, address); }
}

fn get_cpu_info() -> (&str, &str, Option<u32>) {
    return ("ARM32", "Cortex-m", None);
}

pub fn cpu_kernel_init() {
    let (arch, name, address) = get_cpu_info();
    set_cpu_info(arch, name, address);
}
