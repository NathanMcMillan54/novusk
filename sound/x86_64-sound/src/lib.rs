#![no_std]

use pc_beeper;
pub use pc_beeper::Speaker;

// This doesn't make a noise in Qemu, it probably won't on a real machine
pub fn beep() {
    let mut speaker = Speaker::new();
    speaker.beep(1000, 10);
}
