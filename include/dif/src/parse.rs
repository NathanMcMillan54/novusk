use core::fmt::Arguments;
use crate::{Dif, DifFieldNames};

impl Dif {
    pub fn set_and_parse(&mut self, file: &[(&'static str, &'static str); 11]) -> Dif {
        for line in 0..file.len() {
            for field in 0..Dif::DIF_FIELD_NAMES.len() {
                if file[line].0 == Dif::DIF_FIELD_NAMES[field].to_str() {
                    self.set_index(line + 1, (Dif::DIF_FIELD_NAMES[field], file[line].1));
                }
            }
        }

        for i in 1..11 {
            if self.get_index(i).0 == DifFieldNames::DifName {
                self.dif_name = Some(self.get_index(i).1);
            }
        }

        return *self;
    }
}
