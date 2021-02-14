use super::out_str::Writer;

const WRITER: Writer = Writer;

fn clear_screen() {
    let mut lines = 0;
    while lines != 13 {
        WRITER.write_byte(b"\n");
        lines = lines + 1;
    }
}

pub fn cmdline_init() {
    clear_screen();
}
