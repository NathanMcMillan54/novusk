use uefi::prelude::ResultExt;
use uefi::proto::console::text::{Output, OutputMode};

pub(crate) fn text_init(stdout: &mut Output) {
    let text_mode = stdout
        .modes()
        .last()
        .unwrap()
        .expect("There was a warning while getting the text mode");

    stdout
        .set_mode(text_mode)
        .expect_success("Failed to change text mode");
}

pub fn text_mode(stdout: &mut Output) -> Option<OutputMode> {
    return stdout.current_mode().unwrap_success();
}
