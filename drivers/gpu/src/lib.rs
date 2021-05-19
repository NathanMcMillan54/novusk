#![no_std]

#[macro_use] extern crate kerror;

pub static mut INIT: bool = false;
pub static mut GPU: &'static str = "";

#[no_mangle]
pub unsafe extern "C" fn gpu_init() {
    if GPU == "" && INIT == false {
        kerror!("GPU never set or initialized, will not initialize GPU");
    } else if GPU == "" && INIT == true {
        kerror!("GPU never set but was initialized, you should set a GPU");
    }

    if GPU == "gop" {
        INIT = true;
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_gpu(gpu: &'static str) {
    GPU = gpu;
}

#[no_mangle]
pub unsafe extern "C" fn set_init() {
    INIT = true;
}

#[no_mangle]
pub unsafe extern "C" fn gpu_name() -> &'static str {
    return GPU;
}
