use _include::c_ulong;

type rsize_t = c_ulong;

#[no_mangle]
pub extern "C" fn __use_types(_: rsize_t) {
    unimplemented!()
}
