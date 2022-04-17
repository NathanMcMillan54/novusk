pub mod macros;
pub mod names;

use macros::define_core_function;
use names::CoreFunctionNames;

fn impl_empty(_arg: *mut u8) -> *mut u8 {
    return 0x0 as *mut u8;
}

define_core_function!(CoreFunctionNames::empty, arg: *mut u8, -> *mut u8, impl_empty);
