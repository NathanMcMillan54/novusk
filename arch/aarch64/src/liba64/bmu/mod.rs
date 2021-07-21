use crate::kernel::debug::DebugPrint;

static mut NAME: &'static str = "";
static mut MAIN: fn() = main_placeholder;

fn main_placeholder() {

}

pub trait Application {
    unsafe fn new(&mut self, name: &'static str, main: fn()) {
        let mut dprint = DebugPrint;

        NAME = name;
        MAIN = main;

        dprint.write_string("Starting ");
        dprint.write_string(name);
        dprint.write_string("...\n");
    }

    unsafe fn start(&mut self) {
        MAIN();
    }
}
