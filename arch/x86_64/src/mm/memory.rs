use bootloader::bootinfo::{BootInfo, MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{FrameAllocator, OffsetPageTable, PageTable, PhysFrame, Size4KiB};

pub static mut PHYSICAL_MEM_OFFSET: u64 = 0;

pub unsafe fn memory_init(pmo_as_virtaddr: VirtAddr, pmo_as_u64: u64) -> OffsetPageTable<'static> {
    PHYSICAL_MEM_OFFSET = pmo_as_u64;

    let level_4_table = active_level_4_table(pmo_as_virtaddr);
    return OffsetPageTable::new(level_4_table, pmo_as_virtaddr);
}

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    return &mut *page_table_ptr;
}

pub struct EmptyFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for EmptyFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootInfoFrameAllocator {
    pub memory_map: &'static MemoryMap,
    pub next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(mem_map: &'static MemoryMap) -> Self {
        return BootInfoFrameAllocator {
            memory_map: mem_map,
            next: 0,
        };
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
        return frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)));
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        return frame;
    }
}
