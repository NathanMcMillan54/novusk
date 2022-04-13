use core::fmt::Write;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KStatus {
    pub status: &'static str,
    pub should_panic: bool,
    pub panic_message: Option<&'static str>,
    pub message1: &'static str,
    pub message2: Option<&'static str>,
}

impl KStatus {
    pub fn display(&self) {
        printk!("INFO [ {} ] {}\n", self.status, self.message1);

        if self.message2.is_some() {
            printk!("    {}\n", self.message2.unwrap());
        }

        if self.should_panic {
            if self.panic_message.is_some() {
                panic!("{}", self.panic_message.unwrap());
            } else { panic!("A problem occured"); }
        }
    }
}
