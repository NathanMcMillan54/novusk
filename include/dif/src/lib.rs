#![no_std]
#![allow(warnings)]

pub mod fields;
pub mod parse;

use fields::DifFieldNames;
use crate::fields::DifLine;

#[derive(Copy, Clone, PartialEq)]
pub struct Dif {
    pub dif_name: Option<&'static str>,
    pub line1: DifLine,
    pub line2: DifLine,
    pub line3: DifLine,
    pub line4: DifLine,
    pub line5: DifLine,
    pub line6: DifLine,
    pub line7: DifLine,
    pub line8: DifLine,
    pub line9: DifLine,
    pub line10: DifLine,
    pub line11: DifLine,
}

impl Dif {
    pub const DIF_FIELD_NAMES: &'static [DifFieldNames; 18] = &[
        DifFieldNames::None,
        DifFieldNames::DifName,
        DifFieldNames::DeviceName,
        DifFieldNames::BootMethod,
        DifFieldNames::PeripheralAddress,
        DifFieldNames::SocName,
        DifFieldNames::AllocMemory,
        DifFieldNames::EnableSerial,
        DifFieldNames::EnableFrameBuffer,
        DifFieldNames::PrintingMethod,
        DifFieldNames::IrqMethod,
        DifFieldNames::EnableDeviceIrqs,
        DifFieldNames::DeviceSpecificKernel,
        DifFieldNames::StartInit,
        DifFieldNames::InitInput,
        DifFieldNames::InitFs,
        DifFieldNames::InitNet,
        DifFieldNames::ShutdownOnPanic
    ];

    pub const fn empty() -> Self {
        return Dif {
            dif_name: None,
            line1: (DifFieldNames::None, ""),
            line2: (DifFieldNames::None, ""),
            line3: (DifFieldNames::None, ""),
            line4: (DifFieldNames::None, ""),
            line5: (DifFieldNames::None, ""),
            line6: (DifFieldNames::None, ""),
            line7: (DifFieldNames::None, ""),
            line8: (DifFieldNames::None, ""),
            line9: (DifFieldNames::None, ""),
            line10: (DifFieldNames::None, ""),
            line11: (DifFieldNames::None, ""),
        };
    }

    /// Indexes ``Dif`` like an array (start from ``0``), the "array" starts on ``dif_name``
    pub fn get_index(&self, index: usize) -> DifLine {
        return match index {
            0 => (DifFieldNames::DifName, self.dif_name.unwrap_or("unknown.dif")),
            1 => self.line1,
            2 => self.line2,
            3 => self.line3,
            4 => self.line4,
            5 => self.line5,
            6 => self.line6,
            7 => self.line7,
            8 => self.line8,
            9 => self.line9,
            10 => self.line10,
            11 => self.line11,
            _ => (DifFieldNames::None, "Index is expected to be 0 - 11"),
        };
    }

    pub fn set_index(&mut self, index: usize, line: DifLine) {
        match index {
            0 => self.dif_name = Some(line.1),
            1 => self.line1 = line,
            2 => self.line2 = line,
            3 => self.line3 = line,
            4 => self.line4 = line,
            5 => self.line5 = line,
            6 => self.line6 = line,
            7 => self.line7 = line,
            8 => self.line8 = line,
            9 => self.line9 = line,
            10 => self.line10 = line,
            11 => self.line11 = line,
            _ => return,
        };
    }
}
