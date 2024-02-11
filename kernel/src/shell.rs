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

use crate::initrd::INITRD;
use crate::logger::LOGGER;
use crate::serial::SERIAL;
use crate::timer::TIMER;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::{Result, Write};
use core::str;
use spin::{Lazy, Mutex};

pub const BLUE: &str = "\x1b[38;2;0;151;230m";
pub const BOLD: &str = "\x1b[1m";
pub const DEFAULT: &str = "\x1b[39m";
pub const GREEN: &str = "\x1b[38;2;68;189;50m";
pub const NORMAL: &str = "\x1b[0m";
pub const ORANGE: &str = "\x1b[38;2;194;54;52m";
pub const PURPLE: &str = "\x1b[38;2;140;122;230m";
pub const RED: &str = "\x1b[38;2;232;65;24m";
pub const YELLOW: &str = "\x1b[38;2;251;197;49m";

pub struct Shell {
    buffer: Vec<char>,
    prompt: String,
}

impl Shell {
    pub fn new() -> Shell {
        let hostname = str::from_utf8(INITRD.get_data("initrd/etc/hostname"))
            .unwrap()
            .trim_end();
        let username = str::from_utf8(INITRD.get_data("initrd/etc/passwd"))
            .unwrap()
            .split_terminator(':')
            .next()
            .unwrap();
        Shell {
            buffer: Vec::new(),
            prompt: format!("{BOLD}{DEFAULT}[{GREEN}{username}@{hostname} {BLUE}/{DEFAULT}]# "),
        }
    }

    pub fn display<T: Write>(&self, writer: &Lazy<Mutex<T>>) -> Result {
        let motd = str::from_utf8(INITRD.get_data("initrd/etc/motd")).unwrap();
        let mut lock = writer.lock();
        writeln!(lock, "{motd}")?;
        write!(lock, "{}", self.prompt)?;
        Ok(())
    }

    pub fn interpret(&mut self) -> Result {
        let mut serial = SERIAL.lock();
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
                    writeln!(serial, "{NORMAL}")?;
                    match command {
                        "echo" => {
                            let argument = match input {
                                Some(pair) => pair.1,
                                None => "",
                            };
                            writeln!(serial, "{argument}")?;
                        }
                        "help" => {
                            writeln!(serial, "Available commands:")?;
                            writeln!(serial, "\techo -- Display a line of text.")?;
                            writeln!(serial, "\thelp -- Print a list of commands.")?;
                            writeln!(serial, "\tlogs -- Retrieve the system logs.")?;
                            writeln!(serial, "\ttime -- Display the elapsed time.")?;
                            writeln!(
                                serial,
                                "\tpwd  -- Print the name of the current working directory."
                            )?;
                        }
                        "logs" => {
                            for log in LOGGER.lock().get_logs() {
                                writeln!(serial, "{log}")?;
                            }
                        }
                        "pwd" => {
                            writeln!(serial, "/")?;
                        }
                        "time" => {
                            writeln!(serial, "{}", TIMER.lock().get_elapsed())?;
                        }
                        _ => {
                            writeln!(serial, "{RED}ERROR: Command not found.")?;
                        }
                    }
                    write!(serial, "{}", self.prompt)?;
                    self.buffer.clear();
                }
                '\x08' => {
                    if !self.buffer.is_empty() {
                        self.buffer.pop();
                        write!(serial, "{character} {character}")?;
                    }
                }
                _ => {
                    self.buffer.push(character);
                    write!(serial, "{character}")?;
                }
            }
        }
    }
}

pub fn initialize() {
    let mut shell = Shell::new();
    shell
        .display(&SERIAL)
        .expect("Failed to display serial console.");
    shell.interpret().expect("Failed to interpret shell input.");
}
