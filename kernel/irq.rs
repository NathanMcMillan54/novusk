extern "C" {
    pub(crate) fn device_specific_irq_init();
}

macro_rules! define_dev_irq_init {
    ($init_fun:ident) => {
        #[no_mangle]
        pub extern "C" fn device_specific_irq_init() {
            unsafe { $init_fun(); }
        }
    };
}
