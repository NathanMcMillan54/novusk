global_asm!(include_str!("die.S"));

#[cfg(target_arch = "arm")]
#[cfg(feature = "nrf")]
pub mod init;

extern "C" {
    fn die() -> !;
}
