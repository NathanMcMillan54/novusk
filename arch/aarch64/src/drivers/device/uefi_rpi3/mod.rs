use crate::drivers::device::{DEVICE_INFO, Device};

pub unsafe fn uefi_pi3_init() {
    DEVICE_INFO = Device {
        board: "UEFI RPi3",
        manufacture: "Raspberry Pi Foundation",
        cpu: "Cortex A53",
        uart0: 0 as *mut u8,
        main_kernel: true,
        arch_kernel: true
    }
}
