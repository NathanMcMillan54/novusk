use core::ops::Add;
use asminc::aarch64::ints::{enable_irqs, disable_irqs};
use asminc::aarch64::io::{inb, outb};
use bcm::bcm2837::SOC_INFO;
use novuskinc::kernel::types::KernelFunctionName;
use novuskinc::irq::{IrqChip, IrqHandler};

#[link_name = "IRQCHIP"]
#[no_mangle]
pub static mut IRQCHIP: IrqChip = IrqChip {
    name: "BCM2837 IRQ Chip",
    irq_address: 0,
    enabled: false,
    disable: disable_irqs,
    enable: enable_irqs,
    irqn: get_irqn,
    handlers: &mut []
};

#[no_mangle]
unsafe extern "C" fn get_irqn() -> i16 {
    let peripheral_addr = SOC_INFO.get("Peripheral Address").unwrap();
    let irq_addr = peripheral_addr + SOC_INFO.get("IRQ Offset").unwrap();

    return inb(irq_addr.add(4)) as i16;
}

pub unsafe fn bcm2837_irqchip_init() -> i16 {
    IRQCHIP.set_handler((irqs::TIMER1, handlers::TIMER1_irq));

    let peripheral_addr = SOC_INFO.get("Peripheral Address").unwrap();
    let irq_addr = peripheral_addr + SOC_INFO.get("IRQ Offset").unwrap();

    outb(irq_addr.add(0x10), irqs::TIMER1 as u16);

    (IRQCHIP.enable)();
    IRQCHIP.enabled = true;

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

    IRQCHIP.irq_address = irq_address;

    0
}

define_kernel_function!(KernelFunctionName::irqchip_setup, -> i16, bcm2837_irqchip_setup);

#[no_mangle]
pub unsafe extern "C" fn device_irq_handler(irnq: i16) {
    match irnq {
        irqs::TIMER1 => {
            handlers::TIMER1_irq();
        },
        _ => handlers::DefaultHandler(irnq as usize),
    }

    return;
}

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
    use asminc::aarch64::io::{inb, outb};
    use core::ops::Add;
    use bcm::bcm2837::SOC_INFO;
    use bcm::bcm2837::timer::BCM2837_TIMER;

    #[no_mangle]
    pub unsafe extern "C" fn TIMER1_irq() -> i16 {
        BCM2837_TIMER.value += BCM2837_TIMER.interval;

        let peripheral_addr = SOC_INFO.get("Peripheral Address").unwrap();
        let timer = peripheral_addr + SOC_INFO.get("Timer CS Offset").unwrap();
        let timer_c1 = timer.add(0x10);

        outb(timer_c1, BCM2837_TIMER.value as u16);
        outb(timer, 2);

        printk!(".");
        0
    }

    #[no_mangle]
    pub extern "C" fn DefaultHandler(ex: usize) {
        printk!("--- Default Exception Handler ---");
        printk!("| Exception {} was used", ex);
    }
}
