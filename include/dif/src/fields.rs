#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DifFieldNames {
    None,
    DifName,
    DeviceName,
    BootMethod,
    PeripheralAddress,
    SocName,
    AllocMemory,
    EnableSerial,
    EnableFrameBuffer,
    PrintingMethod,
    IrqMethod,
    EnableDeviceIrqs,
    DeviceSpecificKernel,
    StartInit,
    InitInput,
    InitFs,
    InitNet,
    ShutdownOnPanic,
}

impl DifFieldNames {
    pub fn to_str(&self) -> &str {
        return match self {
            DifFieldNames::None => "None",
            DifFieldNames::DifName => "DifName",
            DifFieldNames::DeviceName => "DeviceName",
            DifFieldNames::BootMethod => "BootMethod",
            DifFieldNames::PeripheralAddress => "PeripheralAddress",
            DifFieldNames::SocName => "SocName",
            DifFieldNames::AllocMemory => "AllocMemory",
            DifFieldNames::EnableSerial => "EnableSerial",
            DifFieldNames::EnableFrameBuffer => "EnableFrameBuffer",
            DifFieldNames::PrintingMethod => "PrintingMethod",
            DifFieldNames::IrqMethod => "IrqMethod",
            DifFieldNames::EnableDeviceIrqs => "EnableDeviceIrqs",
            DifFieldNames::DeviceSpecificKernel => "DeviceSpecificKernel",
            DifFieldNames::StartInit => "StartInit",
            DifFieldNames::InitInput => "InitInput",
            DifFieldNames::InitFs => "InitFs",
            DifFieldNames::InitNet => "InitNet",
            DifFieldNames::ShutdownOnPanic => "ShutdownOnPanic",
        };
    }
}

pub type DifLine = (DifFieldNames, &'static str);
