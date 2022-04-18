use printk::Printk;

#[no_mangle]
pub static mut PRINTK: Printk = Printk::new();
