use x86_64::VirtAddr;
use x86_64::instructions::{segmentation, tables};
use x86_64::structures::{gdt::*, tss::TaskStateSegment};

struct Selectors {
    pub kernel_ss: SegmentSelector,
    pub task_ss: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        // Task state segment struct
        let mut ts3 = TaskStateSegment::new();
        ts3.interrupt_stack_table[0 as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        ts3
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdts = GlobalDescriptorTable::new();
        let kernel_ss = gdts.add_entry(Descriptor::kernel_code_segment());
        let tss_ss = gdts.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdts,
            Selectors {
                kernel_ss: kernel_ss,
                task_ss: tss_ss
            },
        )
    };
}

pub unsafe fn gdt_init() {
    GDT.0.load();
    segmentation::set_cs(GDT.1.kernel_ss);
    tables::load_tss(GDT.1.task_ss);
}
