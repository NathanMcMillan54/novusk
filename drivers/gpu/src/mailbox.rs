#[cfg(target_arch = "aarch64")]
pub mod a64_mb {
    use core::sync::atomic::{compiler_fence, Ordering};
    use rpi::common::RpiMb;

    static mut A64MB: RpiMb = RpiMb { mb_buffer: [0; 36] };

    pub unsafe fn aarch64_mailbox_call(mb_do: u8, mailbox_num: u8, mailbox_arg: u8) -> u8 {
        let mut ret = 0;

        // Add to mailbox
        if mb_do == 1 {
            A64MB.mb_buffer[mailbox_num as usize] = mailbox_arg as u32;
        } /* Call the mailbox */ else if mb_do == 2 {
            compiler_fence(Ordering::Release);

            if A64MB.call(mailbox_arg as u32).is_ok() {
                ret = 0;
            } else { ret = 1; }
        }

        return ret;
    }

    //define_syscall!(sys_mailbox_do, aarch64_mailbox_call);
}
