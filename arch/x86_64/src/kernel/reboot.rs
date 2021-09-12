use crate::include::asm::nop;
use x86_64::instructions::port::Port;

pub unsafe fn reboot() -> ! {
    Port::new(0x64).write(0xfeu8);

    printk!("Didn't reboot properly, waiting a few cycles before panicking...");
    for c in 0..100000000 { }

    panic!("Couldn't reboot from kernel");
}
