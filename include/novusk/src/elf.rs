use core::mem;

extern "C" {
    /// Loads an ELF executable from the given path and returns it's entry address.
    pub fn load_elf(name: &'static str) -> u64;
}

#[cfg(target_pointer_width = "64")]
/// Run the ELF from it's entry address (``main`` or ``_start`` function).
pub unsafe fn run_elf(entry: u64) {
    let entry_point: extern "C" fn() = mem::transmute(entry);

    (entry_point)();
}

#[cfg(target_pointer_width = "32")]
/// Run the ELF from it's entry address (``main`` or ``_start`` function).
pub unsafe fn run_elf(entry: u32) {
    let entry_point: extern "C" fn() = mem::transmute(entry);

    (entry_point)();
}
