use alloc::vec::Vec;
use core::fmt::Arguments;
use crate::{str_to_addr, Dif};
use crate::str_to_addr::str_to_addr;

extern "C" {
    pub(crate) fn _early_printk(print: Arguments);
    pub(crate) fn trim_line(line: &'static str, field_name: &'static str) -> &'static str;
}

impl Dif {
    pub fn parse_and_set(&mut self, file: &'static str) -> Dif {
        let mut file_feilds: Vec<&str> = file.split("\n").collect();

        const MINIMUM_FEILDS: usize = 11;
        const REPLACE_CHARS: &[char; 3] = &['\"', ',', ':'];

        let mut gen_dif = Dif::empty();

        if file_feilds.len() != MINIMUM_FEILDS {
            panic!("DIF missing minumun feilds. Has {} feilds, needs {}", file_feilds.len(), MINIMUM_FEILDS);
        }

        if !file_feilds[0].is_ascii() {
            panic!("String line 1 dosen't contain ASCII charater[s], {}", file_feilds[0]);
        } else if !file_feilds[10].is_ascii() {
            panic!("String line 10 doesn't contain ASCII character[s], {}", file_feilds[10]);
        }

        for f in 2..9 {
            if !file_feilds[f].starts_with("0x") {
                panic!("Addresss on line {} doesn't start with 0x", f);
            }
        }

        let mut name = file_feilds[0];
        let mut periph = str_to_addr(file_feilds[1]);
        let mut gpio0 = str_to_addr(file_feilds[2]);
        let mut gpio1 = str_to_addr(file_feilds[3]);
        let mut gpio2 = str_to_addr(file_feilds[4]);
        let mut gpio3 = str_to_addr(file_feilds[5]);
        let mut gpio4 = str_to_addr(file_feilds[6]);
        let mut serial = str_to_addr(file_feilds[7]);
        let mut uart = str_to_addr(file_feilds[8]);
        let mut fb = str_to_addr(file_feilds[9]);
        let mut mb = str_to_addr(file_feilds[10]);
        let mut debug = false;

        if file_feilds[10] == "debug_on" {
            debug = true;
        } else if file_feilds[10] == "debug_off" {
            debug = false;
        } else { panic!("Line 11 needs to be \"debug_on\" or \"debug_off\", {}", file_feilds[10]) }

        gen_dif.device_name = name;
        gen_dif.peripheral_addr = Some(periph);
        gen_dif.gpio0_addr = Some(gpio0);
        gen_dif.gpio1_addr = Some(gpio1);
        gen_dif.gpio2_addr = Some(gpio2);
        gen_dif.gpio3_addr = Some(gpio3);
        gen_dif.gpio4_addr = Some(gpio4);
        gen_dif.serial_addr = Some(serial);
        gen_dif.uart_addr = Some(uart);
        gen_dif.fb_addr = Some(fb);
        gen_dif.mb_addr = Some(mb);
        gen_dif.debug = Some(debug);

        *self = gen_dif;
        return *self;
    }
}
