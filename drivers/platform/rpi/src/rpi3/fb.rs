use core::ptr::write_volatile;
use core::sync::atomic::{compiler_fence, Ordering};
use bcm::bcm2837::mailbox::*;
use novuskinc::firmware::*;

pub fn get_rpi3_fb() -> Option<*mut u32> {
    let mut mb = Bcm2837Mailbox::new();
    // .init();

    mb.buffer[0] = 35 * 4;
    mb.buffer[1] = REQUEST;
    mb.buffer[2] = tag::SETPHYWH;
    mb.buffer[3] = 8;
    mb.buffer[4] = 8;
    mb.buffer[5] = 1024;
    mb.buffer[6] = 768;
    mb.buffer[7] = tag::SETVIRTWH;
    mb.buffer[8] = 8;
    mb.buffer[9] = 8;
    mb.buffer[10] = 1024;
    mb.buffer[11] = 768;
    mb.buffer[12] = tag::SETVIRTOFFSET;
    mb.buffer[13] = 8;
    mb.buffer[14] = 8;
    mb.buffer[15] = 0;
    mb.buffer[16] = 0;
    mb.buffer[17] = tag::SETDEPTH;
    mb.buffer[18] = 4;
    mb.buffer[19] = 4;
    mb.buffer[20] = 32;
    mb.buffer[21] = tag::SETPXORDER;
    mb.buffer[22] = 4;
    mb.buffer[23] = 4;
    mb.buffer[24] = 0;
    mb.buffer[25] = tag::GETFB;
    mb.buffer[26] = 8;
    mb.buffer[27] = 8;
    mb.buffer[28] = 4096;
    mb.buffer[29] = 0;
    mb.buffer[30] = tag::GETPITCH;
    mb.buffer[31] = 4;
    mb.buffer[32] = 4;
    mb.buffer[33] = 0;
    mb.buffer[34] = tag::LAST;

    compiler_fence(Ordering::Release);

    let mb_ret = mb.mb_call(channel::PROP);

    if mb_ret.is_err() {
        return None;
    } else { return Some(mb.buffer[28] as *mut u32); }
}
