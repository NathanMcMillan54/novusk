use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::borrow::{Borrow, BorrowMut};
use core::fmt::Arguments;
use crate::{Dif, libcopy};

extern "C" {
    pub(crate) fn _early_printk(print: Arguments);
    pub(crate) fn trim_line(line: &'static str, field_name: &'static str) -> &'static str;
}

static mut LOCAL_DIF: Dif = Dif::empty();

impl Dif {
    pub fn parse_and_set(&mut self, file: &'static str) -> Dif {
        let mut file_feilds: Vec<&str> = file.split("\n").collect();

        const MINIMUM_FEILDS: usize = 11;
        const REPLACE_CHARS: &[char; 3] = &['\"', ',', ':'];

        if file_feilds.len() != MINIMUM_FEILDS {
            panic!("DIF missing minumun feilds. Has {} feilds, needs {}", file_feilds.len(), MINIMUM_FEILDS);
        }

        if file_feilds[0].is_ascii() {
            self.device_name = file_feilds[0];
        } else { panic!("Name line dosen't contain ASCII charater[s], name: {}", file_feilds[0]); }

        // Stack overflow is down as of rn lol
        // https://stackoverflow.com/questions/49203561/how-do-i-convert-a-str-to-a-const-u8
        /* if file_feilds[1].parse::<i32>().is_ok() {
            self.peripheral_addr = Some(file_feilds[1].parse::<u32>().unwrap());
        } else { panic!("Peripheral line cannot be converted to u32, peripherla_addr: {}", file_feilds[1]); }*/

        for gpio in 2..6 {

        }

        unsafe { return LOCAL_DIF; }
    }
}
