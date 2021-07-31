#[no_mangle]
pub static mut MAILBOX: [usize; 36] = [0, 0, 0 ,0 ,0 , 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

pub unsafe fn clear_mb() {
    for i in 0..36 {
        MAILBOX[i] = 0
    };
}
