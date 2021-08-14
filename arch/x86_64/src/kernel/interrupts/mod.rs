use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptDescriptorTable;

pub mod handler;
pub mod index;
use index::InterruptIndex;
pub mod interrupts;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = 40;

pub static mut PIC: Mutex<ChainedPics> = unsafe { Mutex::new(ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)) };

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idts = InterruptDescriptorTable::new();
        idts.breakpoint.set_handler_fn(handler::break_point_handler);
        idts.page_fault.set_handler_fn(handler::page_fault_handler);
        unsafe { idts.double_fault.set_handler_fn(handler::double_fault_handler).set_stack_index(0); }
        idts[InterruptIndex::Keyboard as usize].set_handler_fn(interrupts::keyboard_interrupt);
        idts[InterruptIndex::Mouse as usize].set_handler_fn(interrupts::mouse_interrupt);
        idts[InterruptIndex::Timer as usize].set_handler_fn(interrupts::time_interrupt);
        idts
    };
}

pub fn idt_init() {
    IDT.load();
}
