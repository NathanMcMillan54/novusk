use r0::zero_bss;

pub unsafe fn clear_bss_se(mut start: u64, mut end: u64) {
    zero_bss(&mut start, &mut end);
}
