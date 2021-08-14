use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

pub mod handler;
pub mod index;
pub mod interrupts;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = 40;

pub static mut PIC: Mutex<ChainedPics> = unsafe { Mutex::new(ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)) };

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(handler::break_point_handler);
        idt.page_fault.set_handler_fn(handler::page_fault_handler);
        unsafe { idt.double_fault.set_handler_fn(handler::double_fault_handler).set_stack_index(0); }
        idt
    };
}

pub fn idt_init() {
    IDT.load();
}
