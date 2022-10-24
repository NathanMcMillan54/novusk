use core::sync::atomic::{compiler_fence, Ordering};
use novuskinc::firmware::FirmwareInterface;
use bcm_soc::mb::*;

#[no_mangle]
pub unsafe extern "C" fn device_display_info() -> ((u32, u32), usize) {
    let mut buffer = [0; 36];
    let mut mb = Bcm2837Mb::new();

    mb.mb_buffer[0] = 35 * 4;
    mb.mb_buffer[1] = REQUEST;

    mb.mb_buffer[2] = tag::SET_PHYSWH;
    mb.mb_buffer[3] = 8;
    mb.mb_buffer[4] = 8;
    mb.mb_buffer[5] = 1024;
    mb.mb_buffer[6] = 768;

    mb.mb_buffer[7] = tag::SET_VIRTWH;
    mb.mb_buffer[8] = 8;
    mb.mb_buffer[9] = 8;
    mb.mb_buffer[10] = 1024;
    mb.mb_buffer[11] = 768;

    mb.mb_buffer[12] = tag::SET_VIRTOFFSET;
    mb.mb_buffer[13] = 8;
    mb.mb_buffer[14] = 8;
    mb.mb_buffer[15] = 0;
    mb.mb_buffer[16] = 0;

    mb.mb_buffer[17] = tag::SET_DEPTH;
    mb.mb_buffer[18] = 4;
    mb.mb_buffer[19] = 4;
    mb.mb_buffer[20] = 32;

    mb.mb_buffer[21] = tag::SET_PXLORDER;
    mb.mb_buffer[22] = 4;
    mb.mb_buffer[23] = 4;
    mb.mb_buffer[24] = 0;

    mb.mb_buffer[25] = tag::GET_FB;
    mb.mb_buffer[26] = 8;
    mb.mb_buffer[27] = 8;
    mb.mb_buffer[28] = 4096;
    mb.mb_buffer[29] = 0;

    mb.mb_buffer[30] = tag::GET_PITCH;
    mb.mb_buffer[31] = 4;
    mb.mb_buffer[32] = 4;
    mb.mb_buffer[33] = 0;

    mb.mb_buffer[34] = tag::LAST;

    compiler_fence(Ordering::Release);
    mb.mb_call(channel::PROP);

    let width = mb.mb_buffer[5];
    let height = mb.mb_buffer[6];
    let fb_addr = mb.mb_buffer[28];

    return ((width, height), fb_addr as usize);
}
