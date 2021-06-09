use super::{device_init, Device};
use cpu::arch::Arch;
use cpu::cpu::Cpu;

pub unsafe fn qemu_virt_init() {
    let dev = Device {
        board: "Virt",
        manufacture: "Qemu",
        cpu_arch: Arch::Aarch64,
        cpu_brand: Cpu::Unknown,
        main_kernel: false,
        arch_kernel: false
    };

    device_init(dev);
}
