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
use core::fmt;
use core::fmt::Write;
use spin::{Lazy, Mutex};

pub static LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| {
    let logger = Logger::new();
    Mutex::new(logger)
});

#[macro_export]
macro_rules! debug {
    ($message:expr) => {
        LOGGER.lock().debug($message);
    };
}

#[macro_export]
macro_rules! error {
    ($message:expr) => {
        LOGGER.lock().error($message);
    };
}

#[macro_export]
macro_rules! info {
    ($message:expr) => {
        LOGGER.lock().info($message);
    };
}

#[macro_export]
macro_rules! trace {
    ($message:expr) => {
        LOGGER.lock().trace($message);
    };
}

#[macro_export]
macro_rules! warn {
    ($message:expr) => {
        LOGGER.lock().warn($message);
    };
}

pub struct Logger {
    line_number: usize,
}

impl Logger {
    pub fn new() -> Logger {
        Logger { line_number: 1 }
    }

    pub fn debug(&mut self, message: &str) {
        self.log("[DEBUG] ", message, Color::Green)
            .expect("Failed to log debug message.");
    }

    pub fn error(&mut self, message: &str) {
        self.log("[ERROR] ", message, Color::Red)
            .expect("Failed to log error message.");
    }

    pub fn info(&mut self, message: &str) {
        self.log("[INFO] ", message, Color::Blue)
            .expect("Failed to log info message.");
    }

    pub fn trace(&mut self, message: &str) {
        self.log("[TRACE] ", message, Color::Purple)
            .expect("Failed to log trace message.");
    }

    pub fn warn(&mut self, message: &str) {
        self.log("[WARN] ", message, Color::Yellow)
            .expect("Failed to log warning message.");
    }

    fn log(&mut self, label: &str, message: &str, label_color: Color) -> fmt::Result {
        let width = VGA.lock().get_width();
        let height = VGA.lock().get_height();
        let font_width = VGA.lock().get_font_width();
        let font_height = VGA.lock().get_font_height();

        VGA.lock().set_cursor(
            width / 3,
            font_height * self.line_number + height - height / 3,
            label_color,
            Color::Black,
        );
        write!(VGA.lock(), "{}", label)?;

        VGA.lock().set_cursor(
            width / 3 + label.len() * font_width,
            font_height * self.line_number + height - height / 3,
            Color::White,
            Color::Black,
        );
        write!(VGA.lock(), "{}", message)?;

        self.line_number += 1;

        Ok(())
    }
}
