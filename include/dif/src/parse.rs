use alloc::vec::Vec;
use core::fmt::Arguments;
use crate::Dif;

impl Dif {
    pub fn parse_and_set(&mut self, file: &[&'static str; 11]) -> Dif {
        let mut new_dif = Dif::empty();

        new_dif.device_name = file[0];
        new_dif.peripheral_addr = Some(file[1].parse::<u32>().unwrap());
        new_dif.gpio1_addr = Some(file[2].parse::<u32>().unwrap());
        new_dif.gpio2_addr = Some(file[3].parse::<u32>().unwrap());
        new_dif.gpio3_addr = Some(file[4].parse::<u32>().unwrap());
        new_dif.gpio4_addr = Some(file[5].parse::<u32>().unwrap());
        new_dif.serial_addr = Some(file[6].parse::<u32>().unwrap());
        new_dif.uart_addr = Some(file[7].parse::<u32>().unwrap());
        new_dif.fb_addr = Some(file[8].parse::<u32>().unwrap());
        new_dif.mb_addr = Some(file[9].parse::<u32>().unwrap());

        // Why does this work?
        if file[10] == "debug_off" {
            new_dif.debug = Some(true);
        } else if file[10] == "debug_on" {
            new_dif.debug = Some(false);
        } else { panic!("Last line should be \"debug_on\" or \"debug_off\", not {}", file[10]); }

        return new_dif;
    }
}
