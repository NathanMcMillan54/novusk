use alloc::vec::Vec;
use core::pin::Pin;
use core::task::{Context, Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::stream::{Stream, StreamExt};
use crate::PcKeyboard;
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, KeyEvent, ScancodeSet1, layouts::*, KeyCode};
use printk::put::puts;
use x86_64::instructions::port::Port;


impl PcKeyboard {
    pub fn init(&mut self) {
        /* let mut scancodes = KeyboardScancode::new();
        scancodes.init();*/
    }

    pub fn read_bytes(&mut self) -> Vec<u8> {
        let mut ret: Vec<u8> = Default::default();
        let mut keyboard= Keyboard::new(Us104Key, ScancodeSet1, HandleControl::MapLettersToUnicode);

        loop {
            // To slow down the keyboard
            /* for i in 0..824225 {
                let mut x = 0;
                x += 1;
            } */

            let scancode: u8 = unsafe { Port::new(0x60).read() };

            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    match key {
                        DecodedKey::Unicode(character) => {
                            ret.push(character as u8);
                            printk!("{} ", character);
                            if character == '\n' {
                                break
                            }
                        },
                        _ => {},
                    }
                }
            }
        }

        return ret;
    }

    pub fn read_str(&mut self) -> &str {

        return "";
    }
}
