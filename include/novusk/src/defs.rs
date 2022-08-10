// Macros for "defining" important functions

#[macro_export]
macro_rules! define_syscall {
    ($sys_name:ident, $syscall_fun:ident) => {
        #[no_mangle]
        pub unsafe extern "C" fn $sys_name(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
            return $syscall_fun(sys_arg1, sys_arg2, sys_arg3);
        }
    };
}

#[macro_export]
macro_rules! define_graphics_pixel {
    ($pixel_fun:ident) => {
        #[no_mangle]
        pub extern "C" pixel(x: usize, y: usize, color: usize) {
            let pixel: fn(usize, usize, usize) = $pixel_fun;
            pixel();
        }
    };
}
