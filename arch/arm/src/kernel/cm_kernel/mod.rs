pub mod irqs;
pub mod print;

use cortex_m_rt::heap_start;
use crate::mm::allocator::allocator_init;
use novuskinc::prelude::early_device_init;
use alloc::vec;
use print::_early_print;

pub fn start_cortex_m_kernel() {
    /*unsafe { allocator_init(heap_start() as usize, 1024); }
    _early_print(format_args!("{}", "Initialized allocator\n"));

    irqs::sys_time_setup();
    _early_print(format_args!("{}", "Setup sys time\n"));*/

    unsafe { early_device_init(); }
    //_early_print(format_args!("{}", "Early device functions initialized\n"));
}
