#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cpu {
    AMD,
    Intel,
    Nvidia,
    Samsung,
    Qualcomm,
    Unknown,
}

// Nvidia, Samsung, and Qualcomm are arm32/aarch64 manufactures
