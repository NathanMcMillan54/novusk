use cpu::CpuInfo;

#[no_mangle]
pub static mut CPUINFO: CpuInfo = CpuInfo::emtpy();
