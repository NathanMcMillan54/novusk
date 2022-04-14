use dif::Dif;

// arch/arm/src/include/dif/difs/rpi3b.dif
#[no_mangle]
pub(crate) static mut DIF: Dif = Dif::empty();
