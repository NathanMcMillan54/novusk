/// This macro is used for defining unimplemented IRQ handlers for Cortex-M CPUs.
#[macro_export]
macro_rules! cm_empty_interrupt {
    ($name:ident) => {
        #[interrupt]
        fn $name() {

        }
    };
}


