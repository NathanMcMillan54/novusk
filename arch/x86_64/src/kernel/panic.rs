use core::panic::PanicInfo;
use vgag::vga::VgaG;
use crate::early_printk;

#[panic_handler]
unsafe fn _panic(_info: &PanicInfo) -> ! {
    early_printk!("\n\nx86_64 kernel panic\n");
    early_printk!("    Message: {}\n", _info.message().unwrap());
    early_printk!("    Location: {}\n", _info.location().unwrap());


    let mut vga = VgaG::new();
    vga.set_mode(0);

    loop { asm!("hlt"); }
}
