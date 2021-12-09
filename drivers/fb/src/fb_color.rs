#[derive(Copy, Clone)]
pub struct FbColor {
    pub r: usize,
    pub g: usize,
    pub b: usize,
}

impl FbColor {
    pub fn new(r: usize, g: usize, b: usize) -> Self {
        return FbColor {
            r: r,
            g: g,
            b: b,
        };
    }
}
