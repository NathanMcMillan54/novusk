use dif::Dif;

pub(crate) const DIF_FILE: &'static str = include_str!("difs/rpi3b.dif");
pub(crate) static mut DIF: Dif = Dif::empty();

pub fn dif_init() {
    unsafe {
        DIF.set(DIF_FILE);
    }
}
