use libcolor::html_colors::HtmlColors;
use crate::mb::*;

pub struct RpiFb {
    pub x: usize,
    pub y: usize,
    pub ptr: usize,
}

impl RpiFb {
    pub fn fb_init() -> Self {
        let mut mailbox = MailBox::new();

        mailbox.mb_buffer[0] = 35 * 4;
        mailbox.mb_buffer[1] = RpiMb::MboxRequest as usize;
        mailbox.mb_buffer[2] = RpiMboxTag::MboxTagSetPhywh as usize;
        mailbox.mb_buffer[3] = 8;
        mailbox.mb_buffer[4] = 8;
        mailbox.mb_buffer[5] = 1920;

        mailbox.mb_buffer[6] = 1080;

        mailbox.mb_buffer[7] = RpiMboxTag::MboxTagSetVirtwh as usize;
        mailbox.mb_buffer[8] = 8;
        mailbox.mb_buffer[9] = 8;
        mailbox.mb_buffer[10] = 1920;
        mailbox.mb_buffer[11] = 1080;

        mailbox.mb_buffer[12] = RpiMboxTag::MboxTagSetVirtoff as usize;
        mailbox.mb_buffer[13] = 8;
        mailbox.mb_buffer[14] = 8;
        mailbox.mb_buffer[15] = 0;
        mailbox.mb_buffer[16] = 0;

        mailbox.mb_buffer[17] = RpiMboxTag::MboxTagSetSetdepth as usize;
        mailbox.mb_buffer[18] = 4;
        mailbox.mb_buffer[19] = 4;
        mailbox.mb_buffer[20] = 32;

        mailbox.mb_buffer[21] = RpiMboxTag::MboxTagSetPxlordr as usize;
        mailbox.mb_buffer[22] = 4;
        mailbox.mb_buffer[23] = 4;
        mailbox.mb_buffer[24] = 1;

        mailbox.mb_buffer[25] = RpiMboxTag::MboxTagGetFb as usize;
        mailbox.mb_buffer[26] = 8;
        mailbox.mb_buffer[27] = 8;
        mailbox.mb_buffer[28] = 4096;
        mailbox.mb_buffer[29] = 0;

        mailbox.mb_buffer[30] = RpiMboxTag::MboxTagGetPitch as usize;
        mailbox.mb_buffer[31] = 4;
        mailbox.mb_buffer[32] = 4;
        mailbox.mb_buffer[33] = 0;

        mailbox.mb_buffer[34] = RpiMboxTag::MboxTagLast as usize;

        mailbox.call(RpiMbCh::MboxChProp as u32);

        return RpiFb {
            x: 1024,
            y: 768,
            ptr: 4096
        }
    }

    pub fn clear_screen(&mut self, color: usize) {
        let mut cursor = self.ptr as *mut usize;

        for y in 0..self.y {
            for x in 0..self.x {
                // TODO: Draw a pixel with a hexdecimal number (html color)
                unsafe {
                    *cursor = color as usize;
                    cursor = cursor.offset(1);
                }
            }
        }
    }
}
