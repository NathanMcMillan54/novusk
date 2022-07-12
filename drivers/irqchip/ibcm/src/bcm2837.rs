use asminc::aarch64::ints::{enable_irqs, disable_irqs};
use asminc::aarch64::io::{inb, outb};
use bcm::bcm2837::SOC_INFO;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::irq::{IrqChip, IrqHandler};

#[no_mangle]
pub static mut BCM2837_IRQCHIP: IrqChip = IrqChip {
    name: "BCM2837 IRQ Chip",
    irq_address: 0,
    enabled: false,
    disable: disable_irqs,
    enable: enable_irqs,
    handlers: &mut []
};

pub unsafe fn bcm2837_irqchip_init() -> i16 {
    BCM2837_IRQCHIP.set_handler((irqs::TIMER1, handlers::TIMER1_irq));

    0
}

define_kernel_function!(KernelFunctionName::irqchip_init, -> i16, bcm2837_irqchip_init);

unsafe fn bcm2837_irqchip_setup() -> i16 {
    let peripheral_address = SOC_INFO.get("Peripheral Address");
    let irq_offset = SOC_INFO.get("IRQ Offset");

    if peripheral_address.is_none() || irq_offset.is_none() {
        return 1;
    }

    let irq_address = peripheral_address.unwrap() + irq_offset.unwrap();

    BCM2837_IRQCHIP.irq_address = irq_address;

    0
}

define_kernel_function!(KernelFunctionName::irqchip_setup, -> i16, bcm2837_irqchip_setup);

pub mod irqs {
    pub const TIMER1: i16 = 1;
    pub const TIMER3: i16 = 3;
    pub const USB: i16 = 9;
    pub const AUX: i16 = 29;
    pub const GPIO0: i16 = 49;
    pub const GPIO1: i16 = 50;
    pub const GPIO2: i16 = 51;
    pub const GPIO3: i16 = 52;
    pub const UART: i16 = 57;
}

pub mod handlers {
    #[no_mangle]
    pub unsafe extern "C" fn TIMER1_irq() -> i16 {

        0
    }

    #[no_mangle]
    pub extern "C" fn DefaultHandler(ex: usize) {
        printk!("--- Default Exception Handler ---");
        printk!("| Exception {} was used", ex);
    }
}
