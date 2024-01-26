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

#[macro_export]
macro_rules! debug {
    ($message:literal) => {
        LOGGER.lock().debug($message);
    };
}

#[macro_export]
macro_rules! error {
    ($message:literal) => {
        LOGGER.lock().error($message);
    };
}

#[macro_export]
macro_rules! info {
    ($message:literal) => {
        LOGGER.lock().info($message);
    };
}

#[macro_export]
macro_rules! warn {
    ($message:literal) => {
        LOGGER.lock().warn($message);
    };
}

pub struct Logger {
    line_number: usize,
}

impl Logger {
    pub fn new() -> Logger {
        let line_number = 1;
        Logger { line_number }
    }

    pub fn debug(&mut self, message: &str) {
        self.log("[DEBUG] ", message, Color::Purple);
    }

    pub fn error(&mut self, message: &str) {
        self.log("[ERROR] ", message, Color::Red);
    }

    pub fn info(&mut self, message: &str) {
        self.log("[INFO] ", message, Color::Blue);
    }

    pub fn warn(&mut self, message: &str) {
        self.log("[WARN] ", message, Color::Yellow);
    }

    fn log(&mut self, label: &str, message: &str, label_color: Color) {
        VGA.write(
            label,
            VGA.get_width() / 3,
            VGA.get_font_height() * self.line_number + VGA.get_height() - VGA.get_height() / 3,
            label_color,
            Color::Black,
        );
        VGA.write(
            message,
            VGA.get_width() / 3 + label.len() * VGA.get_font_width(),
            VGA.get_font_height() * self.line_number + VGA.get_height() - VGA.get_height() / 3,
            Color::White,
            Color::Black,
        );
        self.line_number += 1;
    }
}
