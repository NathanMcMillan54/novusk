use device::riscv::RiscVDevice;
use dif::Dif;

pub static mut RISCV_DEVICE: RiscVDevice = RiscVDevice {
    dif: Dif::new(),
    console: None,
};
