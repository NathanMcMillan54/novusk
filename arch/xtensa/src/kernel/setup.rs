use super::device::initialize_device;
use crate::xprintk;

pub(crate) fn setup_xtensa() {
    if setup_device().is_err() {
        panic!("A problem occurred after device setup");
    } else { unsafe { xprintk!("Successfully initialized {}", super::DEVICE_NAME); } }
}

fn setup_device() -> Result<(), i32> {
    let (success, name) = initialize_device();

    if success.is_err() {
        panic!("Error while setting up {} board: {}", success.err().unwrap(), name);
    }

    unsafe { super::DEVICE_NAME = name; }

    return Ok(())
}
