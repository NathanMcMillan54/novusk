use ::dif::Dif;

pub(crate) mod dif;
use self::dif::{DIF_FILE, DIF};

pub unsafe fn dif_init() {
    DIF.set(DIF_FILE);
}
