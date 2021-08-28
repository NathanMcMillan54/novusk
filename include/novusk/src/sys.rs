#[macro_export]
macro_rules! define_syscall {
    ($sys_fun_name:ident, $sys_fun:path) => {
        #[no_mangle]
        pub extern "C" fn $sys_fun_name(sys_arg: u8) {
            unsafe {
                let sf: fn(u8) = $sys_fun;

                sf(sys_arg);
            }
        }
    };
}
