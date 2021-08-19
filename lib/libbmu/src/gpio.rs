use core::any::Any;

pub trait Gpio {
    fn write<T: Any>(&mut self, pin: T, byte: u8) {

    }

    fn read<T: Any>(&mut self, pin: T) -> u8 {
        return 0;
    }
}
