pub struct KernelTime {
    pub rate: f64,
    time: f64,
}

impl KernelTime {
    pub const fn new() -> Self {
        return KernelTime {
            rate: 0.0,
            time: 0.0,
        }
    }
    
    /// This sets ``rate``'s time, when the timer interrupt is invoked it will increase ``time`` by
    /// ``rate``
    pub fn set_rate(&mut self, rate: f64) {
        self.rate = rate;
    }

    /// Returns the values of ``rate`` and ``time``, ``rate`` is ``0`` and ``time`` is ``1``.
    pub fn kernel_time_info(&self) -> (f64, f64) {
        return (self.rate, self.time)
    }

    /// Updates the value of ``time``, this function should be called by the timer interrupt.
    pub fn update(&mut self) {
        self.time += self.rate;
    }
}

#[no_mangle]
pub static mut KERNEL_TIME: KernelTime = KernelTime::new();

/// Calls ``set_rate`` from ``KernelTime``.
pub unsafe fn set_kernel_time_rate(rate: f64) {
    KERNEL_TIME.set_rate(rate);
}

/// Calls ``update`` from ``KernelTime``.
pub unsafe fn update_time() {
    KERNEL_TIME.update();
}
