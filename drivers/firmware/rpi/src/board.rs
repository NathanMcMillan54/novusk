/*
    When Novusk supports RPi4, https://wiki.osdev.org/Detecting_Raspberry_Pi_Board will be helpful
    for determining the value of MMIO_BASE without features
*/

pub const MMIO_BASE: u32 = 0x3F00_0000;