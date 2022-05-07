use x86_64::VirtAddr;
use x86_64::instructions::tables;
use x86_64::registers::segmentation::{CS, Segment};
use x86_64::structures::{gdt::*, tss::TaskStateSegment};

struct Selectors {
    pub kernel_ss: SegmentSelector,
    pub task_ss: SegmentSelector,
}

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[0 as usize] = {
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
    CS::set_reg(GDT.1.kernel_ss);
    tables::load_tss(GDT.1.task_ss);
}
