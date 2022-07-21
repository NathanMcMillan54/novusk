use core::fmt::{Arguments, Write};
use libcolor::{Color16, ColorCode};
use novuskinc::fb::FrameBufferGraphics;
use spin::Mutex;
use novuskinc::console::KernelConsoleDriver;
use novuskinc::drivers::{names::CONSOLE, Driver, DriverResult};
use novuskinc::keyboard::KeyboardInput;
use novuskinc::led::Led;
use novuskinc::prelude::Storage;

pub mod vga_80x25;

pub trait VgaMode {
    const WIDTH: usize = 0;
    const HEIGHT: usize = 0;
    const BUFFER_ADDRESS: usize = 0x0;

    fn write_byte(&mut self, byte: u8) {    }
}

#[derive(Copy, Clone)]
pub struct ScreenChar {
    character: u8,
    color: ColorCode,
}

pub(crate) const VGAG_NAME: &'static str = "VGAG (VGA Graphics) Driver";
static mut VGA_MODE: u32 = 0;

lazy_static! {
    static ref VGA_80X25: Mutex<vga_80x25::Vga80x25> = Mutex::new(vga_80x25::Vga80x25::new());
}


pub struct VgaG;

impl VgaG {
    pub fn new() -> Self {
        return VgaG;
    }

    pub fn set_mode(&mut self, mode: u32) {
        if mode > 3 {
            panic!("VGA mode cannot be above 3, mode: {}", mode);
        } else { unsafe { VGA_MODE = mode; } }
    }

    fn write(&mut self, s: &str) {
        self.graphics_write_string(s,0, 0);
    }
}

impl FrameBufferGraphics for VgaG {
    fn graphics_write(&self, byte: u8, x: usize, y: usize) {
        unsafe {
            match VGA_MODE {
                0 => { VGA_80X25.lock().write_byte(byte); },
                _ => return,
            }
        }
    }

    fn graphics_write_string(&self, string: &str, x: usize, y: usize) {
        unsafe {
            match VGA_MODE {
                0 => { VGA_80X25.lock().write_str(string); },
                _ => return,
            }
        }
    }

    fn graphics_write_fmt(&self, fmt: Arguments) {
        unsafe {
            match VGA_MODE {
                0 => { VGA_80X25.lock().write_fmt(fmt); },
                _ => return,
            }
        }
    }

    fn graphics_pixel(&self, color: u32, x: u32, y: u32) {

    }
}

impl Write for VgaG {
    fn write_str(&mut self, s: &str) -> core::fmt::Result { Ok(()) }
}

impl KernelConsoleDriver for VgaG {
    fn write_character(&self, c: char, x: u16, y: u16) {
        unsafe {
            match VGA_MODE {
                0 => {
                    VGA_80X25.lock().write_char(c);
                }
                _ => return,
            };
        }
    }

    fn write_string(&self, string: &str, x: u16, y: u16) {
        for b in string.as_bytes() {
            self.write_character(*b as char, x, y);
        }
    }

    fn new_line(&self) {
        unsafe {
            match VGA_MODE {
                0 => VGA_80X25.lock().write_str("\n"),
                _ => return,
            };
        }
    }

    fn clear_screen(&self, option: u16) {
        for _ in 0..VGA_80X25.lock().size.1 {
            self.new_line();
        }
    }

    fn dimensions(&self) -> (u16, u16) {
        unsafe {
            return match VGA_MODE {
                0 => VGA_80X25.lock().size,
                _ => (0, 0),
            };
        }
    }
}

impl KeyboardInput for VgaG {}

impl Led for VgaG {}

impl Storage for VgaG {}

impl Driver for VgaG {
    fn driver_name(&self) -> &'static str {
        return VGAG_NAME;
    }

    fn name(&self) -> &'static str {
        return CONSOLE;
    }

    fn init(&self) -> Option<DriverResult> {
        return Some(Ok(()));
    }
}

/*impl Write for VgaG {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.graphics_write_string(s, 0, 0);
        Ok(())
    }
}*/
