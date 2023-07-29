use crate::boot::early_vga::VGA_WRITER;
use crate::kernel::idt::idt_init;

#[no_mangle]
pub unsafe extern "C" fn setup_arch() {
    idt_init();
    VGA_WRITER.lock().write_string("Initialized IDT\n");
}
