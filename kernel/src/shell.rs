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

use crate::logger::LOGGER;
use crate::serial::SERIAL;
use crate::timer::TIMER;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Result, Write};

pub const BLUE: &str = "\x1b[38;2;0;151;230m";
pub const BOLD: &str = "\x1b[1m";
pub const DEFAULT: &str = "\x1b[39m";
pub const GREEN: &str = "\x1b[38;2;68;189;50m";
pub const NORMAL: &str = "\x1b[0m";
pub const PURPLE: &str = "\x1b[38;2;140;122;230m";
pub const RED: &str = "\x1b[38;2;232;65;24m";
pub const YELLOW: &str = "\x1b[38;2;251;197;49m";

pub struct Shell {
    buffer: Vec<char>,
    prompt: String,
}

impl Shell {
    pub fn new() -> Shell {
        Shell {
            buffer: Vec::new(),
            prompt: format!(
                "\r{}[{}root@localhost {}/{}]# ",
                DEFAULT, GREEN, BLUE, DEFAULT
            ),
        }
    }

    pub fn display(&self) -> Result {
        let mut serial = SERIAL.lock();
        write!(serial, "{}", BOLD)?;
        writeln!(
            serial,
            "{}NeurOS v{} (x86_64)",
            RED,
            env!("CARGO_PKG_VERSION")
        )?;
        writeln!(
            serial,
            "\r{}Copyright (C) 2024 {}",
            BLUE,
            env!("CARGO_PKG_AUTHORS")
        )?;
        writeln!(
            serial,
            "\n\r{}This is an administrative console shell.",
            DEFAULT
        )?;
        writeln!(
            serial,
            "\rTo get started, type the 'help' command (without quotes)."
        )?;
        Ok(())
    }

    pub fn interpret(&mut self) -> Result {
        let mut serial = SERIAL.lock();
        write!(serial, "\n{}", self.prompt)?;
        loop {
            let character = serial.read() as char;
            match character {
                '\r' => {
                    let line: String = self.buffer.iter().collect();
                    let input = line.split_once(char::is_whitespace);
                    let command = match input {
                        Some(pair) => pair.0,
                        None => line.as_str(),
                    };
                    writeln!(serial, "{}", NORMAL)?;
                    match command {
                        "echo" => {
                            let argument = match input {
                                Some(pair) => pair.1,
                                None => "",
                            };
                            writeln!(serial, "\r{}", argument)?;
                        }
                        "help" => {
                            writeln!(serial, "\rAvailable commands:")?;
                            writeln!(serial, "\r\techo -- Display a line of text.")?;
                            writeln!(serial, "\r\thelp -- Print a list of commands.")?;
                            writeln!(serial, "\r\tlogs -- Retrieve the system logs.")?;
                            writeln!(serial, "\r\ttime -- Display the elapsed time.")?;
                            writeln!(
                                serial,
                                "\r\tpwd  -- Print the name of the current working directory."
                            )?;
                        }
                        "logs" => {
                            for log in LOGGER.lock().get_logs() {
                                writeln!(serial, "\r{}", log)?;
                            }
                        }
                        "pwd" => {
                            writeln!(serial, "\r/")?;
                        }
                        "time" => {
                            writeln!(serial, "\r{}", TIMER.lock().get_elapsed())?;
                        }
                        _ => {
                            writeln!(serial, "\r{}ERROR: Command not found.", RED)?;
                        }
                    }
                    write!(serial, "{}{}", BOLD, self.prompt)?;
                    self.buffer.clear();
                }
                '\x08' => {
                    if !self.buffer.is_empty() {
                        self.buffer.pop();
                        write!(serial, "{} {}", character, character)?;
                    }
                }
                _ => {
                    self.buffer.push(character);
                    write!(serial, "{}", character)?
                }
            }
        }
    }
}
