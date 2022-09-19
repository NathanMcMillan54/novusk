use core::arch::asm;
use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::exception;
use crate::cpu::ArmCpuRegisters;
use novuskinc::irq::*;
use novuskinc::syscalls::{arch_syscall, do_syscall};

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
    arch_syscall();
}

static mut SYSTICKS: u64 = 0;
