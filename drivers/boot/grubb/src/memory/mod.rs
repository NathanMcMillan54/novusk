use multiboot2::{BootInformation, BootLoaderNameTag};

pub(crate) fn early_memory_init(bootinfo: &BootInformation) {
    let memory_map = bootinfo.memory_map_tag().unwrap();
    let elf_sections = bootinfo.elf_sections_tag().unwrap();

    let kernel_start_addr = elf_sections.sections().filter(|s| s.is_allocated()).map(|s| s.start_address()).min().unwrap();
    let kernel_end_addr = elf_sections.sections().filter(|e| e.is_allocated()).map(|e| e.end_address()).min().unwrap();
}
