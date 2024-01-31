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
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Result, Write};

const BLUE: &str = "\x1b[38;2;0;151;230m";
const BOLD: &str = "\x1b[1m";
const DEFAULT: &str = "\x1b[39m";
const NORMAL: &str = "\x1b[0m";
const RED: &str = "\x1b[38;2;232;65;24m";

pub fn initialize() -> Result {
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    write!(SERIAL.lock(), "{}", BOLD)?;
    writeln!(SERIAL.lock(), "{}NeurOS v{} (x86_64)", RED, version)?;
    writeln!(SERIAL.lock(), "\r{}Copyright (C) 2024 {}", BLUE, author)?;
    write!(SERIAL.lock(), "\n\r{}> ", DEFAULT)?;
    let mut buffer: Vec<char> = Vec::new();
    loop {
        let character = SERIAL.lock().read() as char;
        match character {
            '\r' => {
                let line: String = buffer.iter().collect();
                let input = line.split_once(char::is_whitespace);
                let command = match input {
                    Some(pair) => pair.0,
                    None => line.as_str(),
                };
                writeln!(SERIAL.lock(), "{}", NORMAL)?;
                match command {
                    "echo" => {
                        let argument = match input {
                            Some(pair) => pair.1,
                            None => "",
                        };
                        writeln!(SERIAL.lock(), "\r{}", argument)?;
                    }
                    "help" => {
                        writeln!(SERIAL.lock(), "\rAvailable commands:")?;
                        writeln!(SERIAL.lock(), "\r\techo -- Display a line of text.")?;
                        writeln!(SERIAL.lock(), "\r\thelp -- Print a list of commands.")?;
                    }
                    _ => {
                        writeln!(SERIAL.lock(), "\r{}ERROR: Command not found.", RED)?;
                    }
                }
                write!(SERIAL.lock(), "\r{}{}> ", BOLD, DEFAULT)?;
                buffer.clear();
            }
            '\x08' => {
                if !buffer.is_empty() {
                    buffer.pop();
                    write!(SERIAL.lock(), "{} {}", character, character)?;
                }
            }
            _ => {
                buffer.push(character);
                write!(SERIAL.lock(), "{}", character)?
            }
        }
    }
}
