use core::mem;

extern "C" {
    pub fn load_elf(name: &'static str) -> u64;
}

pub unsafe fn run_elf(entry: u64) {
    let entry_point: extern "C" fn() = mem::transmute(entry);

    (entry_point)();
}
