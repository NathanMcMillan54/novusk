#[macro_export]
macro_rules! empty_interrupt {
    ($name:ident) => {
        #[interrupt]
        fn $name() {

        }
    };
}