use ::dif::Dif;

pub(crate) mod dif;
use self::dif::{DIF_FILE, DIF};

pub fn dif_init() {
    unsafe { DIF.set(DIF_FILE); }
}
