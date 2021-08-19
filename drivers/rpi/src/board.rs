// Translated from https://wiki.osdev.org/Detecting_Raspberry_Pi_Board
// This doesn't return the proper value
pub unsafe fn check_board() -> u32 {
    let mut board: u32;

    llvm_asm!("mrs $0, midr_el1" : "=r"(board));

    // TODO: When this does work, set MMIO and GPIO addresses for specific boards
    return (board >> 4) & 0xFFF;
}
