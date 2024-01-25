// NeurOS - Hobbyist operating system written in Rust.
// Copyright (C) 2024 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![no_std]
#![no_main]

mod font;
mod image;
mod memory;
mod vga;

use core::panic::PanicInfo;
use font::Font;
use image::Image;
use vga::{Color, Vga};
use x86_64::instructions::{hlt, interrupts};

#[no_mangle]
extern "C" fn _start() -> ! {
    memory::expand();
    let font = Font::new();
    let image = Image::new();
    let vga = Vga::new(font);
    vga.draw_image(image, 128, 256);
    let version = concat!(
        "Version ",
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("COMMIT_HASH"),
        ")"
    );
    vga.write(
        version,
        font.get_width(),
        vga.get_height() - font.get_width(),
        Color::Red,
        Color::Black,
    );
    let copyright = concat!("Copyright (C) 2024 ", env!("CARGO_PKG_AUTHORS"));
    vga.write(
        copyright,
        vga.get_width() - (copyright.len() * font.get_width() + font.get_width()),
        vga.get_height() - font.get_width(),
        Color::Blue,
        Color::Black,
    );
    hcf()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    hcf();
}

pub fn hcf() -> ! {
    interrupts::disable();
    loop {
        hlt();
    }
}
