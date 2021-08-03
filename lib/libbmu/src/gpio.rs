pub trait Gpio {
    fn write<T>(&mut self, pin: T, byte: u8) {

    }

    fn read<T>(&mut self, pin: T) -> u8 {
        return 0;
    }
}
