#[cfg(any(feature = "hifive", feature = "lofive"))]
const DIF_PATH: &'static str = "src/include/dif/e310x.dif";

#[cfg(feature = "virt")]
const DIF_PATH: &'static str = "src/include/dif/virt.dif";

fn main() {
    difi::add_dif(DIF_PATH);
}
