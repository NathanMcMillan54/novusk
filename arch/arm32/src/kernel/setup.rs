use super::device::{device_init, DEVICE_NAME};

pub unsafe fn setup_arm32_kernel() {
    setup_device();
}

unsafe fn setup_device() {
    let (success, name) = device_init();

    match success {
        Err(e) => panic!("Error while initializing device: {:?}", e),
        _ => DEVICE_NAME = name,
    };
}

