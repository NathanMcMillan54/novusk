use crate::DriverNames;

pub static mut GPU_INIT: bool = false;
pub static mut GPU_NAME: DriverNames = DriverNames::None;

pub unsafe fn set_gpu(name: DriverNames) {
    GPU_NAME = name;
}

pub unsafe fn set_init(init: bool) {
    GPU_INIT = init;
}
