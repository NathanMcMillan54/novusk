use bootloader::bootinfo::{MemoryMap, MemoryRegion, FrameRange, MemoryRegionType};
use spin::Mutex;

pub(crate) const BOOT_PHYS_MEM_OFFSET: usize = 1649267441664;

lazy_static! {
    pub(crate) static ref BOOT_MEM_MAP: Mutex<MemoryMap> = Mutex::new(MemoryMap::new());
}

pub fn set_boot_mem_map() {
    BOOT_MEM_MAP.lock().add_region(MemoryRegion { range: FrameRange { start_frame_number: 0x0, end_frame_number: 0x1000 }, region_type: MemoryRegionType::Usable });
}
