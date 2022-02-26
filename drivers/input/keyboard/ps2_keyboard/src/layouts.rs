use pc_keyboard::layouts::*;

pub enum Layout {
    Us104(Us104Key),
    Uk105(Uk105Key),
    Jis109(Jis109Key),
    Dvorak104(Dvorak104Key),
}
