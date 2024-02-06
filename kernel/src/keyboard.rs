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

use crate::keyboard::ScanCode::Enter;
use crate::logger::LOGGER;
use crate::trace;
use crate::vga::{Color, VGA};
use alloc::format;
use core::fmt::Write;
use spin::{Lazy, Mutex};
use x86_64::instructions::port::Port;

pub enum ScanCode {
    Enter = 0x1C,
}

pub static KEYBOARD: Lazy<Mutex<Keyboard>> = Lazy::new(|| {
    let keyboard = Keyboard::new();
    Mutex::new(keyboard)
});

pub struct Keyboard {
    port: Port<u8>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            port: Port::new(0x60),
        }
    }

    pub fn read(&mut self) -> u8 {
        unsafe { self.port.read() }
    }

    pub fn interpret(&mut self) {
        let scan_code = self.read();
        let log = format!("Received scan code (0x{scan_code:x}) from keyboard.");
        trace!(log.as_str());

        if scan_code == Enter as u8 {
            let mut vga = VGA.lock();
            let x = vga.get_font_width();
            let y = vga.get_font_height() + x;
            vga.clear();
            vga.set_cursor(x, y, Color::Yellow as u32, Color::Black as u32);
            write!(vga, "Hello, world!").expect("Failed to write message.");
        }
    }
}
