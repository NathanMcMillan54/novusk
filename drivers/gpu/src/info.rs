pub static mut GPU_INIT: bool = false;
pub static mut GPU_NAME: &'static str = "";

pub unsafe fn set_gpu(name: &'static str) {
    GPU_NAME = name;
}

pub unsafe fn set_init(init: bool) {
    GPU_INIT = init;
}
