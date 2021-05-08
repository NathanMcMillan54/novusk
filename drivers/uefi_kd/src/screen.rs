use uefi::proto::console::text::Output;

pub fn clear_screen(stdout: &mut Output) {
    stdout.clear();
}
