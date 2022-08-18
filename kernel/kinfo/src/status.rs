use crate::{InfoDisplay, Kinfo};

pub static mut KSTATUS: &'static str = "ok";

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KStatus {
    pub status: &'static str,
    pub should_panic: bool,
    pub panic_message: Option<&'static str>,
    pub message1: &'static str,
    pub messages: Option<&'static [&'static str]>,
}

impl InfoDisplay for KStatus {
    fn display_info(&self) {
        printk!("INFO [ {} ] {}\n", self.status, self.message1);

        if self.messages.is_some() {
            for msg in 0..self.messages.unwrap().len() {
                printk!("    {}\n", self.messages.unwrap()[msg]);
            }
        }

        if self.should_panic {
            if self.panic_message.is_some() {
                panic!("{}", self.panic_message.unwrap());
            } else { panic!("A problem occurred"); }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn set_status(status: &'static str) {
    KSTATUS = status;
}

impl Kinfo {
    pub fn status_not_ok(&mut self) -> &'static str {
        self.status = "not ok";
        return self.status;
    }
}
