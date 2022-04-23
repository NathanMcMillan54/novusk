#[macro_export]
macro_rules! aarch64_interrupt {
    ($name:ident, $code:block) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name() {
            $code
        }
    };
}
