use alloc::vec::Vec;
use core::fmt::Write;
use uefi::prelude::*;

pub fn exit_bootservices(image: Handle, mut st: SystemTable<Boot>) {
    st.stdout().write_fmt(format_args!("{}", "Exiting boot services...\n"));
    st.boot_services().stall(5_000_000);

    st.stdout().reset(true).unwrap();

    let mem_sizes = st.boot_services().memory_map_size().map_size;
    let max_mem_size = mem_sizes + 2 * mem_sizes;
    let mut mmap_storage = vec![0; max_mem_size].into_boxed_slice();
    let (st, _iter) = st.exit_boot_services(image, &mut mmap_storage[..]).unwrap();
}
