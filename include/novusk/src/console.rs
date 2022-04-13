/// The KernelConsole struct shoylde be used as a field value of the console driver.
#[derive(Copy, Clone)]
pub struct KernelConsole {
    pub name: &'static str,
    pub width: u16,
    pub height: u16,
    pub chars_written: u64,
}

impl KernelConsole {
    /// Add the name of the console driver with the console dimensions.
    ///
    /// Example:
    /// ```rust
    /// use novuskinc::console::KernelConsole;
    ///
    /// fn console_init() {
    ///     let console = KernelConsole::new("VGA Console", 80, 25);
    /// }
    /// ```
    ///
    /// This kernel console be used for with the VGA text mode because it can fit 80 characters
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
    /// The write_character function is used to write a single character to a certain place
    fn write_character(&self, c: char, x: u16, y: u16) {

    }

    /// The write_string function is used to write a string and should call write_character to write the
    /// individual character
    fn write_string(&self, string: &str, x: u16, y: u16) {

    }

    /// The new_line function is used to move to the next line for the next character to be placed.
    fn new_line(&mut self) {

    }

    /// The clear_screen function is used to clear the kernel console, the option value is an
    /// optional argument, you can use it for whatever you want.
    fn clear_screen(&mut self, option: u16) {

    }

    /// This should return the ``width`` and ``height`` field of KernelConsole.
    fn dimensions(&mut self) -> (u16, u16) {
        return (0, 0);
    }
}
