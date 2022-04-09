use core::arch::asm;

pub struct Time;

impl Time {
    pub fn new() -> Self {
        return Time;
    }

    pub fn sleepc(&mut self, cycles: i64) {
        for i in 0..cycles {
            unsafe { asm!("nop"); }
        }
    }
}
