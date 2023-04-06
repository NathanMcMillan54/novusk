use dif::{Dif, to_DifFieldNames};

extern "C" {
    static DIF: Dif;
}

/// Implementation for ``get_dif_value`` from ``novuskinc``
#[no_mangle]
pub unsafe extern "C" fn get_dif_value(field: &'static str) -> &'static str {
    DIF.get(to_DifFieldNames(field))
}
