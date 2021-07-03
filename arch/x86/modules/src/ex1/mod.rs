global_asm!(include_str!("add.S"));

extern "C" {
    fn add(num1: u32, num2: u32) -> u32;
}

pub unsafe fn ex1_init() {
    let sum = add(1, 2);
}