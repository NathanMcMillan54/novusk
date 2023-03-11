#[no_mangle]
extern "C" fn irqchip_setup() {}

pub(crate) struct Aarch64Kernel {
    pub early: bool,
}

impl Aarch64Kernel {
    pub fn new() -> Self {
        return Aarch64Kernel {
            early: true,
        };
    }

    pub fn setup(&self) {

    }
}

pub(crate) static mut AARCH64_KERNEL: Aarch64Kernel = Aarch64Kernel {
    early: true,
};
