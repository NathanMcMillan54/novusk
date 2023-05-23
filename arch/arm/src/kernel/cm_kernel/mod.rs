pub mod irqs;
pub mod print;

use novuskinc::prelude::early_device_init;
use print::_early_print;

pub fn start_cortex_m_kernel() {
    irqs::sys_time_setup();
    _early_print(format_args!("{}", "Setup sys time\n"));

    unsafe { early_device_init(); }

    _early_print(format_args!("{}", "Early device functions initialized\n"));
}
