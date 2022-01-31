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

impl Dif {
    pub fn parse_and_set(&mut self, file: &'static str) -> Dif {
        let mut file_feilds: Vec<&str> = file.split("\n").collect();

        const MINIMUM_FEILDS: usize = 12;
        const REPLACE_CHARS: &[char; 3] = &['\"', ',', ':'];

        if file_feilds.len() - 2 > MINIMUM_FEILDS {
            panic!("DIF doesn't have the minumum number of feilds, has {}, needs atleast {}", file_feilds.len(), MINIMUM_FEILDS);
        }

        if !file_feilds[1].contains("\"name\":") {
            panic!("Expected line 2 to contatin the name feild");
        }

        let mut name = "";

        let periph_addr = 0x0;

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
