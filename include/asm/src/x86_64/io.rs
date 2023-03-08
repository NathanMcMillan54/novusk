use core::arch::asm;

pub unsafe fn outb(port: u32, out: u16) {
    asm!("outb %al, %dx", in("al") out as u8, in("dx") port, options(att_syntax));
}

pub unsafe fn inb(port: u32) -> u32 {
    let mut in_ret: u8 = 0;
    asm!("inb %dx, %al", in("dx") port, out("al") in_ret, options(att_syntax));

    return in_ret as u32;
}
