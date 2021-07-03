#![no_std]
#![no_main]

extern crate novusk;
use novusk::libcolor::vga_colors::Color;
use novusk::x86::drivers::vga::pixel::_pixel;
use novusk::x86::drivers::vga::VGA_ADDRESS;
use novusk::x86::drivers::vga::vga_80x25::*;
use novusk::x86::x86_printk;
use core::fmt::Write;

fn red_bar(start: usize, end: usize) {
    for i in start..end {
        _pixel(Color::Red, i);
    }
}

fn white_bar(start: usize, end: usize) {
    for i in start..end {
        _pixel(Color::White, i)
    }
}

#[no_mangle]
pub unsafe extern "C" fn kernel_main() -> ! {
    let mut main_writer = Vga80x25 {
        column_position: 21,
        // Brown because it's close to orange
        color_code: ColorCode::new(Color::Brown, Color::White),
        buffer: &mut *(VGA_ADDRESS as *mut Buffer)
    };

    let mut other_writer = Vga80x25 {
        column_position: 21,
        // Brown because it's close to orange
        color_code: ColorCode::new(Color::Black, Color::White),
        buffer: &mut *(VGA_ADDRESS as *mut Buffer)
    };

    red_bar(0, 20);
    white_bar(20, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 40);
    red_bar(40, 41);
    white_bar(41, 60);
    red_bar(60, 80);
    x86_printk!("");

    // Start drawing the leaf
    red_bar(0, 20);
    white_bar(20, 39);
    red_bar(39, 42);
    white_bar(42, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 38);
    red_bar(38, 43);
    white_bar(43, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 37);
    red_bar(37, 44);
    white_bar(44, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 32);
    red_bar(32, 33);
    white_bar(33, 36);
    red_bar(36, 45);
    white_bar(45, 48);
    red_bar(48, 49);
    white_bar(49, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 31);
    red_bar(31, 34);
    white_bar(34, 35);
    red_bar(35, 46);
    white_bar(46, 47);
    red_bar(47, 50);
    white_bar(50, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 30);
    red_bar(30, 35);
    white_bar(35, 35);
    red_bar(34, 47);
    white_bar(46, 46);
    red_bar(46, 51);
    white_bar(51, 60);
    red_bar(60, 80);
    x86_printk!("");


    // Start drawing the stem
    red_bar(0, 20);
    white_bar(20, 29);
    red_bar(29, 52);
    white_bar(52, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 28);
    red_bar(28, 53);
    white_bar(53, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 26);
    red_bar(26, 55);
    white_bar(55, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 27);
    red_bar(27, 54);
    white_bar(54, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 28);
    red_bar(28, 53);
    white_bar(53, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 29);
    red_bar(29, 52);
    white_bar(52, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 27);
    red_bar(27, 54);
    white_bar(54, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 26);
    red_bar(26, 55);
    white_bar(55, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 40);
    red_bar(40, 41);
    white_bar(41, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 40);
    red_bar(40, 41);
    white_bar(41, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 40);
    red_bar(40, 41);
    white_bar(41, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 40);
    red_bar(40, 41);
    white_bar(41, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 40);
    red_bar(40, 41);
    white_bar(41, 60);
    red_bar(60, 80);
    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 60);
    red_bar(60, 80);

    main_writer.write_fmt(format_args!("{}", "Happy Late Canada Day! 2021/7/3"));

    x86_printk!("");

    red_bar(0, 20);
    white_bar(20, 21);
    white_bar(40, 60);
    red_bar(60, 80);

    other_writer.write_fmt(format_args!("{}", "Novusk Pixel/Graphic Example"));

    x86_printk!("");


    red_bar(0, 20);
    white_bar(20, 60);
    red_bar(60, 80);

    loop {  }
}
