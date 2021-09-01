#[macro_export]
macro_rules! app_main {
    ($main:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn kernel_main() -> ! {
            let m = $main;
            m();
        }
    };
}
