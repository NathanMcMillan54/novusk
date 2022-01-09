pub trait SdCard {
    const SD_OK: u32 = 0;
    const SD_ERR: u32 = 0;

    fn status(&self, mask: i32) -> u32 {
        0
    }
}
