global_asm!(include_str!("boot.S"));

pub(crate) mod io;
pub(crate) mod memory;

use io::RPiIo;
use memory::rpi_memory_init;

extern "C" {
    fn binit() -> !;
}

#[no_mangle]
pub unsafe extern "C" fn rpi_setup() -> ! {
    rpi_memory_init();

    RPiIo::init(&RPiIo);

    binit();
}
