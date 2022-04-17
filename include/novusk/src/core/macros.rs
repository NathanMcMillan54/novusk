use super::names::CoreFunctionNames;
pub use crate::define_core_function;

/// ``define_core_function`` is a macro used to define "core" functions (important kernel functions)
/// in kernel modules.
///
/// The first argument needs to be a value from ``CoreFunctionNames`` for the function name that
/// isn't already being used, the next two arguments are the function's arguments (with names) and
/// return type. [This](link) has lots of documentation on all the core functions, how they should
/// be defined, what they should be used for, and a little bit about how certain functions can be
/// implemented.
#[macro_export]
macro_rules! define_core_function {
    ($cfn:ident::$name:ident, $($args:ident:$types:ty),*, -> $ret:ty, $impl_fun:ident) => {
        /// This this is a core function, you can find more information about it [here](link)
        #[no_mangle]
        pub unsafe extern "C" fn $name($($args:$types,)*) -> $ret {
            $impl_fun($($args,)*)
        }
    };
}
