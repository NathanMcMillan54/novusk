struct SysCall {
    pub num: i32,
    pub name: &'static str,
    pub entry: fn(&[u8]) -> (),
}

impl SysCall {
    pub unsafe fn syscall(&self, sys_arg: &[u8]) {
        let mut num = self.num;
        let mut name = self.name;

        match num {
            0 =>
                if name == "write" {
                    (self.entry)(sys_arg)
                } else { self.unknown(); }
            1 => if name == "read" {
                    (self.entry)(sys_arg)
                } else { self.unknown(); }
            _ => self.unknown()
        }
    }

    pub fn unknown(&self) {
        // ('_') <( You don't know what you're doing )
    }
}
