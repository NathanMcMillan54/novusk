static mut KTIME: u32 = 0;

pub unsafe fn start_kernel_clock() {
    KTIME += 1;
}

pub fn kernel_time() -> u32 {
    unsafe {
        KTIME += 1;
        return KTIME;
    }
}
