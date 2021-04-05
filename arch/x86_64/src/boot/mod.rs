pub mod bios;
pub mod cpu;
mod main;

pub struct BiosRegs {
    pub al: u8,
    pub ah: u8,
    pub eax2: u8,
    pub eax3: u8
}

extern "C" {
    pub(crate) fn kernel_init();
    pub(crate) fn kernel_main() -> !;
    pub(crate) fn intcall(int_no: u8, bios_ireg: u8, bios_oreg: u8);
}
