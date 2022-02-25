#[macro_export]
macro_rules! gen_x86_int {
    ($int_name:ident, $int_fun:ident) => {
        #[no_mangle]
        pub extern "x86-interrupt" fn $int_name(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
            unsafe { $int_fun(stack_frame); }
        }
    };
}
