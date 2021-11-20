use crate::{kernel::setup::setup_xtensa, xpirntk};

#[entry]
fn main() -> ! {

    setup_xtensa();

    loop { }
}
