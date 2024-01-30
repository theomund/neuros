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

use crate::serial::SERIAL;
use core::fmt::{Result, Write};

const BLUE: &str = "\x1b[38;2;0;151;230m";
const BOLD: &str = "\x1b[1m";
const DEFAULT: &str = "\x1b[39m";
const RED: &str = "\x1b[38;2;232;65;24m";

pub fn initialize() -> Result {
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    write!(SERIAL.lock(), "{}", BOLD)?;
    write!(SERIAL.lock(), "{}NeurOS v{} (x86_64)\n\r", RED, version)?;
    write!(SERIAL.lock(), "{}Copyright (C) 2024 {}\n\n\r", BLUE, author)?;
    write!(SERIAL.lock(), "{}> ", DEFAULT)?;
    loop {
        let character = SERIAL.lock().read();
        match character {
            b'\r' => {
                write!(SERIAL.lock(), "\n\r{}ERROR: Command not found.\n\r", RED)?;
                write!(SERIAL.lock(), "{}> ", DEFAULT)?;
            },
            _ => write!(SERIAL.lock(), "{}", character as char)?,
        }
    }
}
