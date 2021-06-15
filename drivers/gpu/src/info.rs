pub static mut GPU_NAME: &'static str = "None";
pub static mut GPU_INIT: bool = false;

#[no_mangle]
pub unsafe extern "C" fn set_gpu(name: &'static str) {
    GPU_NAME = name;
}

#[no_mangle]
pub unsafe extern "C" fn gpu_name() -> &'static str {
    return GPU_NAME;
}
