extern "C" {
    /// Disables IRQs.
    pub fn disable_irqs();
    /// Enables IRQs.
    pub fn enable_irqs();
}

/// ``ArchIrqs`` are used to enable or disable IRQs and let the kernel know if IRQs are disabled or
/// enabled.
pub struct ArchIrqs {
    enabled: bool,
}

impl ArchIrqs {
    /// Returns the value of ``self.enabled``.
    pub fn is_enabled(&self) -> bool {
        return self.enabled;
    }

    /// Calls ``enable_irqs`` and sets ``self.enabled`` to ``true``.
    pub unsafe fn enable(&mut self) {
        enable_irqs();
        self.enabled = true;
    }

    /// Calls ``disable_irqs`` and sets ``self.enabled`` to ``false``.
    pub unsafe fn disable(&mut self) {
        disable_irqs();
        self.enabled = false;
    }

    /// Enable IRQs if they're disabled.
    pub unsafe fn enable_if_disabled(&mut self) {
        if !self.is_enabled() {
            self.enable();
        } else { return; }
    }

    /// Disable IRQs if they're enabled.
    pub unsafe fn disable_if_enabled(&mut self) {
        if self.is_enabled() {
            self.disable();
        } else { return; }
    }
}
