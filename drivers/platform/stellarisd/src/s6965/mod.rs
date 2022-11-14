pub mod irqs;
pub mod uart;

pub(crate) fn lm3s6965_init() {

}

pub(crate) mod timer_names {
    pub const TIMER_0A: &'static str = "TIMER0A int";
}
