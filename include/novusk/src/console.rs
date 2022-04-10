/// The KernelConsole struct shoylde be used as a field value of the console driver.
pub struct KernelConsole {
    pub name: &'static str,
    pub width: u16,
    pub height: u16,
    pub chars_written: u64,
}

impl KernelConsole {
    /// Add the name of the console drivver with the console demisions.
    ///
    /// Examplw:
    /// ```rust
    /// use novuskinc::console::KernelConsole;
    ///
    /// fn console_init() {
    ///     let console = KernelConsole::new("VGA Console", 80, 25);
    /// }
    /// ```
    ///
    /// This kernrl console be used for with the VGA text mode because it can fit 80 characters
    /// across and 25 characters up and down.
    pub fn new(console_name: &'static str, console_width: u16, console_height: u16) -> Self {
        return KernelConsole {
            name: console_name,
            width: console_width,
            height: console_height,
            chars_written: 0,
        };
    }
}

/// The console driver has to implement KernelConsoleDriver, the functions the driver implements
/// gets called by the kernel
pub trait KernelConsoleDriver {
    /// The write_char fuction is used to write a single character to a certian place
    fn write_char(&mut self, c: char, x: u16, y: u16) {

    }

    /// The write_str function is used to write a string and should call write_char to write the
    /// individual character
    fn write_str(&mut self, string: &str, x: u16, y: u16) {

    }
}
