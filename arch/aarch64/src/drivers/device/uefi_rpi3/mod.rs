use crate::drivers::device::{device_init, Device};
use cpu::arch::Arch;
use cpu::cpu::Cpu;

pub unsafe fn uefi_pi3_init() {
    let dev = Device {
        board: "UEFI RPi3",
        manufacture: "Raspberry Pi Foundation",
        cpu_arch: Arch::Aarch64,
        cpu_brand: Cpu::ARM_Holdings,
        main_kernel: true,
        arch_kernel: true
    };
    device_init(dev);
}
