use core::cell::Cell;
use novuskinc::memory::KernelMemory;

pub static mut KERNEL_MEMORY: KernelMemory = KernelMemory {
    memory: Cell::new([(0, 0); 16]),
};

pub unsafe fn get_mem() -> (u64, u64) {
    KERNEL_MEMORY.memory.get()[0]
}

pub unsafe fn add_boot_memory(mem: (u64, u64)) {
    let mut kmem = KERNEL_MEMORY.memory.get();
    for i in 0..16 {
        if kmem[i].0 == 0 && kmem[i].1 == 0 {
            kmem[i] = mem;
            KERNEL_MEMORY.memory.replace(kmem);
            return;
        }
    }
}

pub unsafe fn check_available() -> u64 {
    let mut bytes = 0;

    let mem = KERNEL_MEMORY.memory.get();

    for i in 0..mem.len() {
        if mem[i].0 != 0 && mem[i].1 != 0 {
            let start = mem[i].0;
            let end = mem[i].1;

            bytes += end - start;
        }
    }

    return bytes;
}
