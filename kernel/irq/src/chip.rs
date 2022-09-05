use novuskinc::irq::IrqChip;

#[no_mangle]
pub static mut IRQCHIP: IrqChip = IrqChip {
    name: "Empty Chip",
    irq_address: 0,
    enabled: false,
    disable: empty_enable_disable,
    enable: empty_enable_disable,
    irqn: empty_irqn,
    handlers: vec![]
};

pub extern "C" fn empty_enable_disable() {}
pub extern "C" fn empty_irqn() -> i16 { 0 }

#[no_mangle]
pub unsafe extern "C" fn set_irqchip(chip: IrqChip) {
    IRQCHIP = chip;
}
