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
    let image = Image::new();
    let width = VGA.lock().get_width();
    let height = VGA.lock().get_height();
    let font_width = VGA.lock().get_font_width();

    VGA.lock()
        .set_cursor(width / 10, height / 4, Color::White, Color::Black);
    VGA.lock().draw_image(image);

    VGA.lock()
        .set_cursor(font_width, height - font_width, Color::Red, Color::Black);
    write!(
        VGA.lock(),
        "Version {} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("COMMIT_HASH")
    )?;

    let copyright = concat!("Copyright (C) 2024 ", env!("CARGO_PKG_AUTHORS"));
    VGA.lock().set_cursor(
        width - (copyright.len() * font_width + font_width),
        height - font_width,
        Color::Blue,
        Color::Black,
    );
    write!(VGA.lock(), "{}", copyright)?;

    info!("The operating system has been successfully initialized.");

    Ok(())
}
