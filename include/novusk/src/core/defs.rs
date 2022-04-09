use super::{CoreArguments, names::CoreFunctionName, CORE_FUNCTIONS};

pub unsafe fn check_core_function(function: (CoreFunctionName, unsafe extern "C" fn(CoreArguments) -> u8)) -> bool {
    let mut already_exists = false;

    for f in 0..CORE_FUNCTIONS.unwrap().len() {
        if function.0 == CORE_FUNCTIONS.unwrap()[f].0 {
            already_exists = true;
        }
    }

    if already_exists {
        panic!("Tried to call a duplicate of a core function");
    } else {
        CORE_FUNCTIONS.unwrap().iter().map({|fun|
            function
        });
        return true
    }

    return false;
}

#[macro_export]
macro_rules! define_core_function {
    ($name:ident, $function_name:path, $arguments:block, $block:block,) => {
        #[no_mangle]
        pub unsafe extern "C" fn $name(arguments: CoreArguments) -> u8 {
            use $crate::core::defs::check_core_function;

            extern "C" {
                static mut CORE_FUNCTIONS: Option<[(CoreFunctionName, unsafe extern "C" fn(CoreArguments) -> u8);]>;
            }
        }
    }
}
