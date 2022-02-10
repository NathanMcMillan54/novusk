// For now this will only handle status
#[macro_export]
macro_rules! kinfo {
    ($status:expr) => { $status.display(); };
}
