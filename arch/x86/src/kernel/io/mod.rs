pub mod input;
pub mod output;

pub unsafe fn io_init() {
    output::output_init();
    input::input_init();
}
