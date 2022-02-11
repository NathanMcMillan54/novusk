pub unsafe fn outb(port: u16, out: u8) {
    asm!("outb %al, %dx", in("al") out, in("dx") port, options(att_syntax));
}

pub unsafe fn inb(port: u16) -> u8 {
    let mut in_ret = 0;
    asm!("inb %dx, %al", in("dx") port, out("al") in_ret, options(att_syntax));

    return in_ret;
}
