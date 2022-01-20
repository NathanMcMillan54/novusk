#[macro_export]
macro_rules! define_syscall {
    ($sys_function:ident, $function_name:ident) => {
        #[no_mangle]
        pub extern "C" fn $sys_function(sys_arg1: u8, sys_arg2: u8, sys_arg3; u8) -> u8 {
            return unsafe { $function_name(sys_arg1, sys_arg2, sys_arg3) };
        }
    };
}

