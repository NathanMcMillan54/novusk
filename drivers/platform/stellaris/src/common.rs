use novuskinc::kernel::types::KernelFunctionName;
use tm4c123x::Peripherals;
use crate::dif::DIF_FILE;
use crate::clocks::setup_sys_clocks;

pub fn stellaris_board_init() {
    if tm4c123x::Peripherals::take().is_none() {
        panic!("Couldn't find device peripherals");
    }

    setup_sys_clocks();
}

fn common_serial_init() -> u8 {
    return 0;
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, common_serial_init);
