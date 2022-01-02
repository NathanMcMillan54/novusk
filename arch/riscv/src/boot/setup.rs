use hifive1::{clock, pin};
use hifive1::stdout;
use hifive1::hal::{DeviceResources};
use hifive1::hal::prelude::_e310x_hal_time_U32Ext;
use crate::rv_printk;
use setup::{BootSetup, SetupReturn};

pub struct RiscvBoot;

impl RiscvBoot {
    pub fn new() -> Self {
        return RiscvBoot;
    }

    pub fn setup(&self) {
        let io_ret = self.early_io_init();
        let ld_mem_ret = unsafe { self.linker_setup() };

        rv_printk!("Finished boot setup\n");
        rv_printk!("{}\n{}\n", io_ret.1, ld_mem_ret.1);

        if io_ret.0.is_err() || ld_mem_ret.0.is_err() {
            panic!("An error occurred during boot setup");
        } else { kinfo!("Boot setup successful\n"); }
    }
}

impl BootSetup for RiscvBoot{
    fn early_io_init(&self) -> SetupReturn {
        // The only devices the riscv kernel supports is sifive this doesn't need to change for a while
        let dev_res = DeviceResources::take().unwrap();
        let peripherals = dev_res.peripherals;
        let pins = dev_res.pins;

        let clock = clock::configure(peripherals.PRCI, peripherals.AONCLK, 320.mhz().into());

        stdout::configure(
            peripherals.UART0,
            pin!(pins, uart0_tx),
            pin!(pins, uart0_rx),
            115_200.bps(),
            clock
        );

        return (Ok(()), "Early I/O initialized");
    }

    unsafe fn linker_setup(&self) -> SetupReturn {
        extern "C" {
            static mut _sbss: u64;
            static mut _ebss: u64;
        }

        r0::zero_bss(_sbss as *mut u64, _ebss as *mut u64);

        return (Ok(()), "Linker-mem setup");
    }
}
