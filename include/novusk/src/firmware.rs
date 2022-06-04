pub const FMI_STATUS_EMPTY: u32 = 0;
pub const FMI_STATUS_FULL: u32 = 1;
pub const FMI_STATUS_OTHER: u32 = 2;
pub const FMI_RESPONSE_SUCCESS: u32 = 0;
pub const FMI_RESPONSE_ERROR: u32 = 1;
pub const FMI_RESPONSE_OTHER: u32 = 2;

pub trait FirmwareInterface {
    fn init(&mut self) {

    }

    fn name(&self) -> &'static str {
        return "";
    }

    fn mb_call(&mut self, channel: u32) -> Result<(), u32> {
        Err(FMI_RESPONSE_ERROR)
    }

    fn status(&self) -> u32 {
        FMI_STATUS_FULL
    }

    fn add_index(&mut self, index: usize, val: u32) {

    }
}

