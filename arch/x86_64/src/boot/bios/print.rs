pub struct BiosPrint;


impl BiosPrint {
    pub unsafe fn bios_print(msg: u8) {
        // llvm_asm!("$0x10");
    }
}
