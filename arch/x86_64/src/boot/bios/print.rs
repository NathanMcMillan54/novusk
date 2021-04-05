pub struct BiosPrint;


impl BiosPrint {
    pub unsafe fn bios_print(&mut self, msg: u8) -> u8 {
        // llvm_asm!("$0x10");
        return msg;
    }

    pub unsafe fn print_byte(&mut self, byte: &'static [u8]) -> &'static [u8] {
        for &char in byte {
            self.bios_print(char);
        }
        return byte;
    }

    pub unsafe fn print_string(&mut self, string: &'static str) -> &'static str {
        for char in string.bytes() {
            self.bios_print(char);
        }
        return string;
    }
}
