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

use crate::vga::{Color, VGA};
use spin::{Lazy, Mutex};

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    let logger = Logger::new();
    Mutex::new(logger)
});

pub struct Logger {
    line_number: usize,
}

impl Logger {
    pub fn new() -> Logger {
        let line_number = 1;
        Logger { line_number }
    }

    pub fn error(&mut self, message: &str) {
        self.log("[ERROR] ", message, Color::Red);
    }

    fn log(&mut self, label: &str, message: &str, label_color: Color) {
        VGA.write(
            label,
            VGA.get_font_width(),
            VGA.get_font_height() * self.line_number + VGA.get_font_width(),
            label_color,
            Color::Black,
        );
        VGA.write(
            message,
            VGA.get_font_width() + (label.len() * VGA.get_font_width()),
            VGA.get_font_height() * self.line_number + VGA.get_font_width(),
            Color::White,
            Color::Black,
        );
        self.line_number += 1;
    }

    pub fn warning(&mut self, message: &str) {
        self.log("[WARNING] ", message, Color::Yellow);
    }
}
