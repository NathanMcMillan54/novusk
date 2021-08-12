// This trait should be implemented with UsbD
pub trait UsbRW {
    fn read(&mut self) -> u8 {
        return 0;
    }

    fn write(&mut self, byte: u8) {

    }
}
