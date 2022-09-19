// Common system calls
/// Write - In kernel
pub const KSYS_WRITE: usize = 0;
/// Write - In user
pub const SYS_WRITE: usize = 1;
/// Console write - In user
pub const SYS_CWRITE: usize = 2;
/// Console write - In kernel
pub const KSYS_CWRITE: usize = 3;
/// File write - In user
pub const SYS_FWRITE: usize = 4;
/// File write - In kernel
pub const KSYS_FWRITE: usize = 5;
/// Read - In user
pub const KSYS_READ: usize = 6;
/// Read - In user
pub const SYS_READ: usize = 7;
/// Key read - In user
pub const SYS_KREAD: usize = 8;
/// Key read - In Kernel
pub const KSYS_KREAD: usize = 9;
/// File read - In user
pub const SYS_FREAD: usize = 10;

/// Sleep - In user
pub const SYS_SLEEP: usize = 20;
/// Exit - In user
pub const SYS_EXIT: usize = 21;
/// Setup console - In user
pub const SYS_SETUP_CONSOLE: usize = 22;
/// Initialize module - In kernel
pub const KSYS_INIT_MODULE: usize = 23;
/// Initialize module - In user
pub const SYS_INIT_MODULE: usize = 24;
/// Add a kernel module - In user
pub const SYS_ADD_KM: usize = 25;
/// Add a kernel module - In kernel
pub const KSYS_ADD_KM: usize = 26;
/// Remove a kernel module - In user
pub const SYS_RM_KM: usize = 27;
/// Remove a kernel module - In kernel
pub const KSYS_RM_KM: usize = 28;
