use core::mem;

extern "C" {
    pub fn load_elf(name: &'static str) -> u64;
}

#[cfg(target_pointer_width = "64")]
pub unsafe fn run_elf(entry: u64) {
    let entry_point: extern "C" fn() = mem::transmute(entry);

    (entry_point)();
}

#[cfg(target_pointer_width = "32")]
pub unsafe fn run_elf(entry: u32) {
    let entry_point: extern "C" fn() = mem::transmute(entry);

    (entry_point)();
}
