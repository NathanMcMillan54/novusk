pub struct Time;

impl Time {
    pub fn new() -> Self {
        return Time;
    }

    pub fn sleepc(cycles: i32) {
        for i in 0..cycles {
            unsafe { asm!("nop"); }
        }
    }
}
