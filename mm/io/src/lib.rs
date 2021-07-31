#![no_std]

pub unsafe fn mmio_write(address: usize, offset: usize, value: u8) {
    let reg = address as *mut u8;

    reg.add(offset).write_volatile(value);
}

pub unsafe fn mmio_read(address: usize, offset: usize, value: u8) -> u8 {
    let reg = address as *mut u8;

    return reg.add(offset).read_volatile();
}
