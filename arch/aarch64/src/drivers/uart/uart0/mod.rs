pub mod dprint;
mod uart;
pub use uart::Uart0;

use spin::Mutex;

lazy_static! {
    pub static ref UART0: Mutex::<Uart0> = Mutex::new(Uart0);
}
