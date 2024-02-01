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
use crate::vga::{Color, VGA};
use core::fmt;
use core::fmt::Write;

pub fn initialize() -> fmt::Result {
    let image = Image::new();
    let image_width = image.get_width();
    let mut vga = VGA.lock();
    let width = vga.get_width();
    let height = vga.get_height();
    let font_width = vga.get_font_width();

    vga.set_cursor(
        (width / 2) - (image_width / 2),
        height / 4,
        Color::White,
        Color::Black,
    );
    vga.draw_image(image);

    vga.set_cursor(font_width, height - font_width, Color::Red, Color::Black);
    write!(
        vga,
        "Version {} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("COMMIT_HASH")
    )?;

    let copyright = concat!("Copyright (C) 2024 ", env!("CARGO_PKG_AUTHORS"));
    vga.set_cursor(
        width - (copyright.len() * font_width + font_width),
        height - font_width,
        Color::Blue,
        Color::Black,
    );
    write!(vga, "{}", copyright)?;

    let instruction = "Press ENTER to continue.";
    vga.set_cursor(
        (width / 2) - (instruction.len() * font_width / 2),
        height - (height / 4),
        Color::White,
        Color::Black,
    );
    write!(vga, "Press ENTER to continue.")?;

    Ok(())
}
