pub type MailBoxResult = Result<(), u32>;

pub trait MailBoxSender {
    fn mailbox_full(&self) -> bool {
        return false;
    }

    fn mailbox_empty(&self) -> bool {
        return false;
    }

    fn mailbox_call(&self, channel: u32) -> MailBoxResult {
        return Err(1);
    }
}
