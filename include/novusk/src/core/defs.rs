#[macro_export]
macro_rules! define_core_function {
    ($core_function:ident, $arguments:ident, $ret:ident $function:ident) => {
        #[no_mangle]
        pub extern "C" fn $core_function($arguments) -> $ret {
            return $functions();
        }
    };
}
