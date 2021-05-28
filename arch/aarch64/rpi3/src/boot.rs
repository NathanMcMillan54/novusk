use arm_lib::include::asm::wfe;
use cortex_a::regs::{MPIDR_EL1, SP, RegisterReadOnly, RegisterReadWrite};
use r0::zero_bss;

use crate::linker::{__bss_end, __bss_start};
use crate::setup;

pub(crate) unsafe fn reset() -> ! {
    zero_bss(&mut __bss_start, &mut __bss_end);
    setup::rpi_setup()
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    const CORE_MASK: u64 = 0x3;
    const STACK_START: u64 = 0x80_000;

    match MPIDR_EL1.get() & CORE_MASK {
        0 => {
            SP.set(STACK_START);
            reset()
        }
        _ => loop {
            wfe()
        }
    }
}
