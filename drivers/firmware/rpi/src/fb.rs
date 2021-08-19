use crate::mb::*;

pub unsafe fn fb_init() {
    let mut mailbox = MailBox::new();

    mailbox.mailbox[0] = 35 * 4;
    mailbox.mailbox[1] = RpiMb::MboxRequest as usize;
    mailbox.mailbox[2] = RpiMboxTag::MboxTagSetPhywh as usize;
    mailbox.mailbox[3] = 8;
    mailbox.mailbox[4] = 8;
    mailbox.mailbox[5] = 1920;

    mailbox.mailbox[6] = 1080;

    mailbox.mailbox[7] = RpiMboxTag::MboxTagSetVirtwh as usize;
    mailbox.mailbox[8] = 8;
    mailbox.mailbox[9] = 8;
    mailbox.mailbox[10] = 1920;
    mailbox.mailbox[11] = 1080;

    mailbox.mailbox[12] = RpiMboxTag::MboxTagSetVirtoff as usize;
    mailbox.mailbox[13] = 8;
    mailbox.mailbox[14] = 8;
    mailbox.mailbox[15] = 0;
    mailbox.mailbox[16] = 0;

    mailbox.mailbox[17] = RpiMboxTag::MboxTagSetSetdepth as usize;
    mailbox.mailbox[18] = 4;
    mailbox.mailbox[19] = 4;
    mailbox.mailbox[20] = 32;

    mailbox.mailbox[21] = RpiMboxTag::MboxTagSetPxlordr as usize;
    mailbox.mailbox[22] = 4;
    mailbox.mailbox[23] = 4;
    mailbox.mailbox[24] = 1;

    mailbox.mailbox[25] = RpiMboxTag::MboxTagGetFb as usize;
    mailbox.mailbox[26] = 8;
    mailbox.mailbox[27] = 8;
    mailbox.mailbox[28] = 4096;
    mailbox.mailbox[29] = 0;

    mailbox.mailbox[30] = RpiMboxTag::MboxTagGetPitch as usize;
    mailbox.mailbox[31] = 4;
    mailbox.mailbox[32] = 4;
    mailbox.mailbox[33] = 0;

    mailbox.mailbox[34] = RpiMboxTag::MboxTagLast as usize;

    mailbox.call()
}
