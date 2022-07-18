use novuskinc::irq::{handle_irq, IRQH_FAILED, IRQH_NOT_EXISTENT, IRQH_SUCCESS};

pub(crate) static mut COUNTED_SYSTICKS: u64 = 0;

pub mod exceptions {
    use cortex_m_rt::{ExceptionFrame, exception};
    use novuskinc::irq::handle_irq;

    #[cfg(feature = "debug")]
    use core::fmt::Write;

    #[exception]
    unsafe fn SysTick() {
        super::COUNTED_SYSTICKS += 1;

        #[cfg(feature = "debug")]
            hio::io::HioWriter::new().write_fmt(format_args!("{}{}{}", "SysTick invoked, COUNTED_SYSTICKS = ", super::COUNTED_SYSTICKS, "\n"));
    }

    #[exception]
    unsafe fn DefaultHandler(irqn: i16) {
        // Try to handle the IRQ again
        let irq_result = handle_irq(irqn);

        match irq_result {
            novuskinc::irq::IRQH_SUCCESS => return,
            novuskinc::irq::IRQH_FAILED => { panic!("Failed to handle irq {}", irqn) },
            _ => {
                printk!("\n--- Default Exception Handler ---\n");
                printk!("| IRQ: {} was invoked\n", irqn);
            }
        }
    }

    // This causes an error in ``cortex-m-rt`` when building
    /* #[exception]
    unsafe fn HardFault(exf: &ExceptionFrame) -> ! {
        printk!("--- Hard Fault Handler ---\n");
        printk!("| Exception Frame: {:?}\n", exf);
        panic!("Hard fault");
    } */
}
