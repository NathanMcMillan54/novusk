//#[cfg(feature = "allocator")]
pub mod allocator;

#[cfg(feature = "cortex_a")]
pub mod mmu;

#[cfg(feature = "cortex_m")]
pub mod mpu;

pub fn early_memory_setup() {
    #[cfg(feature = "cortex_m")]
    mpu::setup_mpu();

    #[cfg(feature = "cortex_a")]
    mmu::setup_mmu();

    cfg_if! {
        if #[cfg(feature = "allocator")] {
            unsafe { allocator::allocator_init(); }
            allocator::test_allocator();
        }
    }
}
