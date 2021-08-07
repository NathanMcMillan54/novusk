use super::rpi_mb::*;

pub unsafe fn fb_init() {
    MAILBOX[0] = 35 * 4;
    MAILBOX[1] = RpiMb::MboxRequest as usize;
    MAILBOX[2] = RpiMboxTag::MboxTagSetPhywh as usize;
    MAILBOX[3] = 8;
    MAILBOX[4] = 8;
    MAILBOX[5] = 1920;

    MAILBOX[6] = 1080;

    MAILBOX[7] = RpiMboxTag::MboxTagSetVirtwh as usize;
    MAILBOX[8] = 8;
    MAILBOX[9] = 8;
    MAILBOX[10] = 1920;
    MAILBOX[11] = 1080;

    MAILBOX[12] = RpiMboxTag::MboxTagSetVirtoff as usize;
    MAILBOX[13] = 8;
    MAILBOX[14] = 8;
    MAILBOX[15] = 0;
    MAILBOX[16] = 0;

    MAILBOX[17] = RpiMboxTag::MboxTagSetSetdepth as usize;
    MAILBOX[18] = 4;
    MAILBOX[19] = 4;
    MAILBOX[20] = 32;

    MAILBOX[21] = RpiMboxTag::MboxTagSetPxlordr as usize;
    MAILBOX[22] = 4;
    MAILBOX[23] = 4;
    MAILBOX[24] = 1;

    MAILBOX[25] = RpiMboxTag::MboxTagGetFb as usize;
    MAILBOX[26] = 8;
    MAILBOX[27] = 8;
    MAILBOX[28] = 4096;
    MAILBOX[29] = 0;

    MAILBOX[30] = RpiMboxTag::MboxTagGetPitch as usize;
    MAILBOX[31] = 4;
    MAILBOX[32] = 4;
    MAILBOX[33] = 0;

    MAILBOX[34] = RpiMboxTag::MboxTagLast as usize;

    // When mail box calling is added, call it with the updated MAILBOX
}
