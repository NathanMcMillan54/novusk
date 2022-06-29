use novuskinc::kernel::types::KernelFunctionName;
use tm4c123x::Peripherals;
use tm4c123x_hal::sysctl::{CrystalFrequency, Oscillator, PllOutputFrequency, SystemClock, SysctlExt};
use crate::dif::DIF_FILE;

fn setup_sys_clocks() {
    let dev_peripherals = unsafe { Peripherals::steal() };

    let mut sctl = dev_peripherals.SYSCTL.constrain();
    sctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_16_67mhz),
    );
}

pub fn stellaris_board_init() {
    if tm4c123x::Peripherals::take().is_none() {
        panic!("Couldn't find device peripherals");
    }

    setup_sys_clocks();
}

mod irqs {
    #[no_mangle]
    pub unsafe extern "C" fn device_specific_irqs_init() {

    }
}


fn common_serial_init() -> u8 {
    return 0;
}

define_kernel_function!(KernelFunctionName::early_serial_init, -> u8, common_serial_init);
