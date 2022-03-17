// For ARMv7+ (novusk should only support "new" hardware)
/* #[cfg(feature = "cortex_m")]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    extern "C" {
        fn DefaultHandler();
    }

    DefaultHandler
}; 240]; */

#[cfg(feature = "cortex_m")]
mod cm_ints {
    use cortex_m_rt::{exception, ExceptionFrame};

    #[exception]
    unsafe fn DefaultHandler(irq: i16) -> ! {
        // hprintln!("IRQ: {}", irq);
        loop { asm!("wfe"); }
    }

    /*#[exception]
    unsafe fn HardFault(_ef: &ExceptionFrame) -> ! {
        loop { }
    }*/
}
