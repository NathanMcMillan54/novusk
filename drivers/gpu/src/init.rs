use crate::info::{GPU_INIT, GPU_NAME};

#[no_mangle]
pub unsafe extern "C" fn gpu_init() {
    if GPU_NAME == "None" && GPU_INIT == false {
        printk!("GPU name never set or initialized, will not initialize GPU");
    } else if GPU_NAME == "None" && GPU_INIT == true {
        printk!("GPU name never set but was initialized, set a graphics driver name");
    }

    if GPU_NAME == "GOP" {
        gop::init::gop_init();
        GPU_INIT = true;
    } else if GPU_NAME == "VGA" {
        GPU_INIT = true;
    } else {
        printk!("GPU name never set, will not initialize");
    }
}
