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

use crate::ansi::{BLUE, BOLD, DEFAULT, GREEN, NORMAL, RED};
use crate::elf::Elf;
use crate::initrd::INITRD;
use crate::logger::LOGGER;
use crate::serial::Serial;
use crate::serial::SERIAL;
use crate::syscall;
use crate::timer::TIMER;
use crate::vga::{Color, VGA};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::{Result, Write};
use core::str;
use spin::{Lazy, Mutex, MutexGuard};

pub static SERIAL_CONSOLE: Lazy<Mutex<Shell>> = Lazy::new(|| {
    let shell = Shell::new();
    Mutex::new(shell)
});

pub static VGA_CONSOLE: Lazy<Mutex<Shell>> = Lazy::new(|| {
    let mut vga = VGA.lock();
    let x = vga.get_font_width();
    let y = vga.get_font_height() + x;
    vga.clear();
    vga.set_cursor(x, y, Color::White as u32, Color::Black as u32);
    let shell = Shell::new();
    Mutex::new(shell)
});

pub struct Shell {
    buffer: Vec<char>,
    group: String,
    group_id: u32,
    prompt: String,
    user_id: u32,
    username: String,
    working_directory: String,
}

impl Shell {
    pub fn new() -> Shell {
        let hostname = str::from_utf8(INITRD.get_data("initrd/etc/hostname"))
            .unwrap()
            .trim_end();
        let mut passwd = str::from_utf8(INITRD.get_data("initrd/etc/passwd"))
            .unwrap()
            .split_terminator(':');
        let username = passwd.nth(0).unwrap();
        let user_id = passwd.nth(1).unwrap().parse().unwrap();
        let group_id = passwd.nth(0).unwrap().parse().unwrap();
        let home = passwd.nth(1).unwrap();
        let mut groups = str::from_utf8(INITRD.get_data("initrd/etc/group"))
            .unwrap()
            .split_terminator(':');
        let group = groups.nth(0).unwrap();
        Shell {
            buffer: Vec::new(),
            group: group.to_string(),
            group_id,
            prompt: format!("{BOLD}{DEFAULT}[{GREEN}{username}@{hostname} {BLUE}~{DEFAULT}]# "),
            user_id,
            username: username.to_string(),
            working_directory: home.to_string(),
        }
    }

    pub fn display<T: Write>(&self, writer: &mut MutexGuard<T>) -> Result {
        let motd = str::from_utf8(INITRD.get_data("initrd/etc/motd")).unwrap();
        writeln!(writer, "{motd}")?;
        write!(writer, "{}", self.prompt)?;
        Ok(())
    }

    pub fn interpret(&mut self, writer: &mut MutexGuard<Serial>) -> Result {
        let character = writer.read() as char;
        match character {
            '\r' => {
                let line: String = self.buffer.iter().collect();
                let input = line.split_once(char::is_whitespace);
                let command = match input {
                    Some(pair) => pair.0,
                    None => line.as_str(),
                };
                writeln!(writer, "{NORMAL}")?;
                match command {
                    "echo" => {
                        let argument = match input {
                            Some(pair) => pair.1,
                            None => "",
                        };
                        writeln!(writer, "{argument}")?;
                    }
                    "fork" => {
                        let pid = syscall::fork();
                        writeln!(writer, "Created child process with ID #{pid}.")?;
                    }
                    "help" => {
                        writeln!(writer, "Available commands:")?;
                        writeln!(writer, "\techo     -- Display a line of text.")?;
                        writeln!(writer, "\tfork     -- Create child process.")?;
                        writeln!(writer, "\thelp     -- Print a list of commands.")?;
                        writeln!(writer, "\tid       -- Print user and group ID.")?;
                        writeln!(writer, "\tlogs     -- Retrieve the system logs.")?;
                        writeln!(writer, "\tpwd      -- Print current working directory.")?;
                        writeln!(writer, "\treadelf  -- Read ELF executable file.")?;
                        writeln!(writer, "\treboot   -- Reboot the operating system.")?;
                        writeln!(writer, "\tshutdown -- Shutdown the operating system.")?;
                        writeln!(writer, "\ttime     -- Display the elapsed time.")?;
                    }
                    "id" => {
                        writeln!(
                            writer,
                            "uid={}({}) gid={}({})",
                            self.user_id, self.username, self.group_id, self.group
                        )?;
                    }
                    "logs" => {
                        for log in LOGGER.lock().get_logs() {
                            writeln!(writer, "{log}")?;
                        }
                    }
                    "pwd" => {
                        writeln!(writer, "{}", self.working_directory)?;
                    }
                    "readelf" => {
                        let argument = match input {
                            Some(pair) => pair.1,
                            None => "",
                        };
                        let executable = Elf::new(argument);
                        writeln!(writer, "{executable}")?;
                    }
                    "reboot" => {
                        writeln!(writer, "Rebooting the operating system.")?;
                    }
                    "shutdown" => {
                        writeln!(writer, "Shutting down the operating system.")?;
                    }
                    "time" => {
                        writeln!(writer, "{}", TIMER.get_elapsed())?;
                    }
                    _ => {
                        writeln!(writer, "{RED}ERROR: Command not found.")?;
                    }
                }
                write!(writer, "{}", self.prompt)?;
                self.buffer.clear();
            }
            '\x08' => {
                if !self.buffer.is_empty() {
                    self.buffer.pop();
                    write!(writer, "{character} {character}")?;
                }
            }
            _ => {
                self.buffer.push(character);
                write!(writer, "{character}")?;
            }
        }
        Ok(())
    }
}

pub fn initialize() {
    SERIAL_CONSOLE
        .lock()
        .display(&mut SERIAL.lock())
        .expect("Failed to display serial console.");
}
