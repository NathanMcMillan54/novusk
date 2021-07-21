use kinfo::info::set_info;
use crate::info::{GPU_INIT, GPU_NAME};
use crate::DriverNames;

pub unsafe fn gpu_init() {
    if GPU_INIT == false && GPU_NAME == DriverNames::None {
        set_info("not ok");
        printk!("GPU never set or initialized");
    } else if GPU_INIT == true && GPU_NAME == DriverNames::None {
        printk!("GPU never set but was initialized, set a GPU");
        return;
    } else if GPU_INIT == true && GPU_NAME != DriverNames::None {
        return;
    }
}
