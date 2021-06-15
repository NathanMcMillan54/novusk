#![no_std]
#![feature(extern_types)]

use libcolor::vga_colors::Color;

extern "C" { pub fn _vga_pixel(color: Color); }

pub unsafe fn _pixel(color: &str) {
    if color == "black" {
        _vga_pixel(Color::Black);
    } else if color == "blue" {
        _vga_pixel(Color::Blue);
    } else if color == "green" {
        _vga_pixel(Color::Green);
    } else if color == "cyan" {
        _vga_pixel(Color::Cyan);
    } else if color == "red" {
        _vga_pixel(Color::Red);
    } else if color == "magenta" {
        _vga_pixel(Color::Magenta);
    } else if color == "brown" {
        _vga_pixel(Color::Brown);
    } else if color == "lightgray" {
        _vga_pixel(Color::LightGray);
    } else if color == "drakgray" {
        _vga_pixel(Color::DarkGray);
    } else if color == "lightgreen" {
        _vga_pixel(Color::LightGreen);
    } else if color == "lightblue" {
        _vga_pixel(Color::LightBlue);
    } else if color == "lightcyan" {
        _vga_pixel(Color::LightCyan);
    } else if color == "lightred" {
        _vga_pixel(Color::LightRed);
    } else if color == "pink" {
        _vga_pixel(Color::Pink);
    } else if color == "yellow" {
        _vga_pixel(Color::Yellow);
    } else if color == "white" {
        _vga_pixel(Color::White);
    } else {
        // Something
    }
}

#[macro_export]
macro_rules! pixel {
    ($($color:tt)*) => {$crate::_pixel($($color)*)};
}
