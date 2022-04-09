pub mod defs;
pub mod names;

use names::CoreFunctionName;

pub struct CoreArguments {
    pub arg0: Option<u8>,
    pub arg1: Option<u16>,
    pub arg3: Option<&'static[u8; 64]>
}

#[no_mangle]
pub static mut CORE_FUNCTIONS: Option<[(CoreFunctionName, unsafe extern "C" fn(CoreArguments) -> u8); 1]> = None;
