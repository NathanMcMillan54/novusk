use x86_64::structures::idt::InterruptStackFrame;

#[macro_export]
macro_rules! gen_x86_int {
    ($int_name:ident) => {
        #[no_mangle]
        pub extern "x86-interrupt" fn int_$int_name(stack_frame: InterruptStackFrame) {
            unsafe { $int_name(stack_frame); }
        }
    };
}
