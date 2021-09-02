global_asm!(include_str!("arm64.S"));

pub mod sys;
pub mod asm {
    pub unsafe fn wfe() -> ! {
        extern "C" { fn wait_for_event() -> !; }
        wait_for_event()
    }
}
