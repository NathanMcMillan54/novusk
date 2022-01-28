use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt::Arguments;
use crate::{Dif, libcopy};
use libcopy::copy_value;

extern "C" {
    pub(crate) fn _early_printk(print: Arguments);
}

fn trim_line(line: &str) -> Cow<str> {
    return Cow::Owned(line.replace(" ", "").replace("\"", "").replace(":", "").replace(",", ""));
}

impl Dif {
    pub fn parse(&mut self, file: &'static str) -> Dif {
        let mut file_feilds: Vec<&str> = file.split("\n").collect();

        const MINIMUM_FEILDS: usize = 12;

        if file_feilds.len() - 2 > MINIMUM_FEILDS {
            panic!("DIF doesn't have the minumum number of feilds");
        }

        let mut name_line = file_feilds[1];
        let mut name = "";

        if !name_line.contains("\"name\"") {
            unsafe { _early_printk(format_args!("{}\n", name_line)) }
            panic!("name_line doesn't include \"name\"");
        } else {
            //name = &trim_line(name_line);
        }

        unsafe { _early_printk(format_args!("{}{}", "name: ", name)) }

        let periph_line = file_feilds[2];
        let periph_addr = 0x0 as u32;

        if !name_line.contains("\"name\"") {
            panic!("\"name\" feild should be the first feild on the second line");
        }

        let mut gen_dif = Dif::new(
            name,
            Some(periph_addr),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None);

        return gen_dif;
    }
}
