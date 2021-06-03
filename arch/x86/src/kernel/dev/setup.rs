use super::DEVICE_INFO;
use crate::drivers::device::device::Device;

pub unsafe fn set_device(device: Device) {
    DEVICE_INFO = {
        device
    }
}
