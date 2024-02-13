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
use crate::info;
use crate::logger::LOGGER;
use crate::vga::{Color, VGA};
use core::fmt;
use core::fmt::Write;

pub fn initialize() -> fmt::Result {
    let mut vga = VGA.lock();
    let width = vga.get_width();
    let height = vga.get_height();
    let font_width = vga.get_font_width();

    let logo = Image::new("initrd/usr/share/images/logo.pbm");
    vga.set_cursor(
        (width / 2) - (logo.get_width() / 2),
        height / 8,
        Color::White as u32,
        Color::Black as u32,
    );
    vga.draw_image(&logo);

    let neuro = Image::new("initrd/usr/share/images/neuro.ppm");
    vga.set_cursor(
        (width / 3) - (neuro.get_width() / 2),
        height / 2,
        Color::White as u32,
        Color::Black as u32,
    );
    vga.draw_image(&neuro);

    let evil = Image::new("initrd/usr/share/images/evil.ppm");
    vga.set_cursor(
        width - (width / 3) - (evil.get_width() / 2),
        height / 2,
        Color::White as u32,
        Color::Black as u32,
    );
    vga.draw_image(&evil);

    vga.set_cursor(
        font_width,
        height - font_width,
        Color::Red as u32,
        Color::Black as u32,
    );
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
        Color::Blue as u32,
        Color::Black as u32,
    );
    write!(vga, "{copyright}")?;

    let instruction = "Press ENTER to continue.";
    vga.set_cursor(
        (width / 2) - (instruction.len() * font_width / 2),
        height - (height / 4),
        Color::White as u32,
        Color::Black as u32,
    );
    write!(vga, "{instruction}").expect("Failed to write instruction.");

    info!("The operating system has been successfully initialized.");

    Ok(())
}
