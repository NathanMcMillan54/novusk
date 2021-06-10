use uefi::proto::console::text::{Output, OutputMode};
use uefi::ResultExt;

pub unsafe fn set_text_mode(stdout: &mut Output) {
    let text_mode = stdout
        .modes()
        .last()
        .unwrap()
        .expect("Warnings encountered while querying text mode");
    stdout
        .set_mode(text_mode)
        .expect_success("Failed to change text mode");
}

pub fn current_text_mode(stdout: &mut Output) -> Option<OutputMode> {
    return stdout.current_mode().unwrap_success();
}
