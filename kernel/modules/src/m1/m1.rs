pub unsafe fn m1_init() {
    printk!("   Module 1 init\n");
}

pub unsafe fn m1_exit() {
    printk!("   Module 1 end\n");
}

pub const MODULE: &str = "m1";
pub const AUTHOR: &str = "Nathan McMillan <nathanmcmillan54@gmail.com>";
