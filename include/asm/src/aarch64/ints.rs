use core::arch::asm;

#[macro_export]
macro_rules! aarch64_interrupt {
    ($name:ident, $code:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            $code
        }
    };
}

pub unsafe extern "C" fn enable_irqs() {
    asm!("msr daifclr, #2");
}

pub unsafe extern "C" fn disable_irqs() {
    asm!("msr daifset, #2");
}
