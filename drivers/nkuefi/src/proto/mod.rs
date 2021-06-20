use uefi::table::{Boot, SystemTable};

pub mod text;

pub fn proto_init(st: SystemTable<Boot>) {
    text::text_init(st.stdout());
}
