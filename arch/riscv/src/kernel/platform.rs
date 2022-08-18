use device::riscv::RiscVDevice;
use dif::Dif;

#[no_mangle]
pub static mut RISCV_DEVICE: RiscVDevice = RiscVDevice {
    dif: Dif::new(),
    console: None,
};
