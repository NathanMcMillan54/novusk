use crate::kernel::out_str::{Writer};
use core::fmt::{Arguments, Write};

pub(crate) fn _aarch64_print(args: Arguments) {
    let mut writer = Writer;
    writer.write_fmt(format_args!("{}{}", args, "\n"));
}
