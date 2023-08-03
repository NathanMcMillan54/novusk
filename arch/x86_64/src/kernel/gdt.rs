use x86_64::instructions::segmentation::Segment;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{CS, SegmentSelector};
use x86_64::structures::{gdt::GlobalDescriptorTable, tss::TaskStateSegment};
use x86_64::structures::gdt::Descriptor;
use x86_64::VirtAddr;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[0] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, SegmentSelector, SegmentSelector) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_segment = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_segment = gdt.add_entry(Descriptor::tss_segment(&TSS));

        (gdt, code_segment, tss_segment)
    };
}

//static mut GDT: GlobalDescriptorTable = GlobalDescriptorTable::new();

pub unsafe fn gdt_init() {
    GDT.0.load();

    CS::set_reg(GDT.1);
    load_tss(GDT.2);
}
