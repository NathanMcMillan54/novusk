pub struct Board {
    pub name: &'static str,
    pub peripheral_addr: usize,
    pub early_printing_method: &'static str,
    pub main_printing_method: &'static str,
    pub arch_init: bool,
    pub kernel_init: bool,
    pub board_specific_kernel: Option<unsafe extern "C" fn()>,
}

impl Board {
    pub const fn empty() -> Self {
        return Board {
            name: "Unknown",
            peripheral_addr: 0x0,
            early_printing_method: "Serial",
            main_printing_method: "Serial",
            arch_init: true,
            kernel_init: true,
            board_specific_kernel: None
        }
    }

    pub fn set(&mut self, board: Board) {
        self.name = board.name;
        self.peripheral_addr = board.peripheral_addr;
        self.early_printing_method = board.early_printing_method;
        self.main_printing_method = board.main_printing_method;
        self.arch_init = board.arch_init;
        self.kernel_init = board.kernel_init;
        self.board_specific_kernel = board.board_specific_kernel;
    }

    pub unsafe fn run_board_specific_kernel(&self) {
        if self.board_specific_kernel.is_none() {
            panic!("Can't find board specific kernel for {}", self.name);
        } else {
            self.board_specific_kernel.unwrap()();
        }
    }
}
