use crate::kernel::types::KernelFunctionName;
pub use crate::define_kernel_function;

/// ``define_kernel_function`` is a macro used to define "kernel" functions (important kernel functions)
/// in kernel modules.
///
/// The first argument needs to be a value from ``kernelFunctionNames`` for the function name that
/// isn't already being used, the next two arguments are the function's arguments (with names) and
/// return type. [This](link) has lots of documentation on all the kernel functions, how they should
/// be defined, what they should be used for, and a little bit about how certain functions can be
/// implemented.
#[macro_export]
macro_rules! define_kernel_function {
    ($cfn:ident::$name:ident, -> $ret:ty, $impl_fun:ident) => {
        /// This is a kernel function, you can find more information about it [here](link)
        #[no_mangle]
        pub unsafe extern "C" fn $name() -> $ret {
            KernelFunctionName::$name;
            $impl_fun()
        }
    };

    ($cfn:ident::$name:ident, $($args:ident:$types:ty),*, -> $ret:ty, $impl_fun:ident) => {
        /// This is a kernel function, you can find more information about it [here](link)
        #[no_mangle]
        pub unsafe extern "C" fn $name($($args:$types,)*) -> $ret {
            KernelFunctionName::$name;
            $impl_fun($($args,)*)
        }
    };
}
