#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
global_asm!(include_str!("time.S"));

pub mod time;

extern "C" {
    pub fn currentTime() -> f32;
}

extern "C" {
    pub fn startTime();
}
