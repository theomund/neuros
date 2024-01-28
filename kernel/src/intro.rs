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

use crate::image::Image;
use crate::logger::LOGGER;
use crate::serial::SERIAL;
use crate::vga::{Color, VGA};
use crate::{debug, info, trace};
use x86_64::instructions;

pub fn initialize() {
    let image = Image::new();
    VGA.draw_image(image, VGA.get_width() / 10, VGA.get_height() / 4);
    let version = concat!(
        "Version ",
        env!("CARGO_PKG_VERSION"),
        " (",
        env!("COMMIT_HASH"),
        ")"
    );
    VGA.write(
        version,
        VGA.get_font_width(),
        VGA.get_height() - VGA.get_font_width(),
        Color::Red,
        Color::Black,
    );
    let copyright = concat!("Copyright (C) 2024 ", env!("CARGO_PKG_AUTHORS"));
    VGA.write(
        copyright,
        VGA.get_width() - (copyright.len() * VGA.get_font_width() + VGA.get_font_width()),
        VGA.get_height() - VGA.get_font_width(),
        Color::Blue,
        Color::Black,
    );
    SERIAL.write(b'>');
    instructions::interrupts::int3();
    debug!("This is a sample debug message.");
    trace!("This is a sample trace message.");
    info!("The operating system has been successfully initialized.");
}
