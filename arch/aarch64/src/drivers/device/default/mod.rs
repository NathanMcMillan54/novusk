use crate::drivers::device::{device_init, Device};
use cpu::arch::Arch;
use cpu::cpu::Cpu;

pub unsafe fn default_init() {
    let dev = Device {
        board: "Default",
        manufacture: "None",
        cpu_arch: Arch::Aarch64,
        cpu_brand: Cpu::AMD,
        main_kernel: false,
        arch_kernel: false
    };

    device_init(dev);
}
