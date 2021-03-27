mod main;

extern "C" {
    pub(crate) fn kernel_init() -> !;
}
