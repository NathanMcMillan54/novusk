pub mod allocator;
pub use allocator::ALLOCATOR;

#[path = "../../../../mm/bss.rs"]
pub(crate) mod bss;
