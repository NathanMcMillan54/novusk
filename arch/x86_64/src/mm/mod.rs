use bootloader::BootInfo;
use x86_64::VirtAddr;

pub mod allocator;

pub fn test_allocator() {
    let mut test_vec = vec![0];

    for _ in 0..1024 {
        test_vec.push(1);
    }
}
