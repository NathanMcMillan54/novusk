#[macro_export]
macro_rules! vga_write {
    ($($arg:tt)*) => {$crate::drivers::vga::_vga_write(format_args!($($arg)*))};
}
