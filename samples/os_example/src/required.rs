use libnu::ktypes::ApplicationType;

#[no_mangle]
pub extern "C" fn kernel_info() -> bool { return true; }

#[no_mangle]
pub extern "C" fn application_type() -> ApplicationType { return ApplicationType::OperatingSystem; }

#[no_mangle]
pub extern "C" fn main_color() -> &'static str { return "cyan"; }

#[no_mangle]
pub extern "C" fn initramfs() -> bool { return false; }

#[no_mangle]
pub extern "C" fn initramfs_main() { return; }
