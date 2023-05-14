use dif::Dif;

extern "C" {
    pub static DIF_FILE: &'static [(&'static str, &'static str)];
}

#[no_mangle]
pub static mut DIF: Dif = Dif::new();

pub unsafe fn set_dif() {
    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }
    DIF = DIF.parse(DIF_FILE);
    unsafe { core::ptr::write_volatile(0x4000_C000 as *mut u8, b'h'); }
}
