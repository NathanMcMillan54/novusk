#[derive(Copy, Clone, PartialEq)]
pub enum CoreFunctionName {
    empty,
    cpu_disable_dwt,
}

pub enum CoreModuleNames {
    early_device_init,
    early_device_end,
    core_display_init,
    core_display_end,
}
