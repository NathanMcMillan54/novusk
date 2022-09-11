use core::arch::asm;
use core::ffi::CStr;
use core::slice::from_raw_parts;
use core::str::{from_utf8_unchecked};
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::exception;
use novuskinc::irq::*;
use novuskinc::syscalls::{arch_syscall, syscall};

#[no_mangle]
pub unsafe extern "C" fn test_exception() -> u8 {
    // Use SysTick to check if exceptions are working
    asm!("wfi");

    if SYSTICKS != 0 {
        return 0;
    } else { return 1; }
}

#[exception]
fn SysTick() {
    unsafe { SYSTICKS += 1; }
}

#[exception]
unsafe fn DefaultHandler(irqn: i16) {
    let irq_ret = handle_irq(irqn);

    match irq_ret {
        IRQH_SUCCESS => return,
        IRQH_FAILED => panic!("IRQ {} failed\n", irqn),
        IRQH_NOT_EXISTENT => printk!("IRQ {} is unimplemented\n", irqn),
        _ => panic!("IRQ returned {}", irq_ret)
    };
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    printk!("Exception frame: {:?}", ef);
    panic!("Hard fault");
}

#[exception]
unsafe fn SVCall() {
    let psp_ptr = cortex_m::register::msp::read() as *const *const u8;
    let psp_array = core::slice::from_raw_parts(psp_ptr, 5);
    printk!("\nsvc {:?}\n", psp_array);

    /* let sys_ret = syscall(psp_array[0] as usize, psp_array);

    cortex_m::register::psp::write(sys_ret.as_ptr() as u32); */
}

static mut SYSTICKS: u64 = 0;
